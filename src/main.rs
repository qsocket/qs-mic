extern crate anyhow;
extern crate cpal;
extern crate hound;

// use anyhow::Ok;
use cpal::traits::{DeviceTrait, HostTrait};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time;

mod options;
mod utils;

fn main() -> anyhow::Result<()> {
    let mut opts = options::parse_options()?;
    unsafe {
        utils::VERBOSE = opts.verbose;
        utils::QUIET = opts.quiet;
    }

    if opts.generate {
        opts.secret = utils::random_secret();
    }

    if opts.secret.is_empty() {
        // Ask for secret.
        opts.secret = utils::read_line("Enter Secret (or press Enter to generate): ");
        if opts.secret.is_empty() {
            opts.secret = utils::random_secret();
        }
    }
    // print option summary
    options::summarize_options(&opts);

    if opts.listen {
        loop {
            match probe_qsrn(&opts) {
                Ok(_) => (),
                Err(e) => {
                    if !e.to_string().contains("Connection refused") {
                        utils::print_error(e.to_string().as_str())
                    }
                }
            }
            std::thread::sleep(time::Duration::from_secs(opts.probe as u64));
        }
    } else {
        connect(&mut opts)?;
    }

    Ok(())
}

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    match format {
        cpal::SampleFormat::U16 => hound::SampleFormat::Int,
        cpal::SampleFormat::I16 => hound::SampleFormat::Int,
        cpal::SampleFormat::F32 => hound::SampleFormat::Float,
    }
}

fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
    hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: sample_format(config.sample_format()),
    }
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: cpal::Sample,
    U: cpal::Sample + hound::Sample,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = cpal::Sample::from(&sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}

fn probe_qsrn(opts: &options::CommandParams) -> Result<(), anyhow::Error> {
    let mut qsock = qsocket::QSocket::new(&opts.secret, opts.verify_cert);
    qsock.add_id_tag(qsocket::peer_id_tag::SERVER)?;
    qsock.add_id_tag(qsocket::peer_id_tag::PROXY)?;
    qsock.dial()?;

    let mut buf: [u8; 1] = [0];
    qsock.read_exact(buf.as_mut())?;

    let fname = temp_file();
    match start_recording(&opts.device.clone(), fname.clone(), buf[0] as u64) {
        Ok(fname) => utils::print_verbose(&format!("Recording to {}", fname.to_str().unwrap())),
        Err(e) => utils::print_error(&e.to_string()),
    };

    let mut file = File::open(fname.clone())?;
    io::copy(&mut file, &mut qsock)?;

    // Delete the temp recording file...
    utils::print_status(&format!("Removing {}", fname.to_str().unwrap()));
    fs::remove_file(fname)?;
    Ok(())
}

fn connect(opts: &mut options::CommandParams) -> anyhow::Result<()> {
    let mut qsock = qsocket::QSocket::new(&opts.secret, opts.verify_cert);
    qsock.add_id_tag(qsocket::peer_id_tag::CLIENT)?;
    qsock.add_id_tag(qsocket::peer_id_tag::PROXY)?;
    qsock.dial()?;

    qsock.write_all(&[opts.rec_duration])?;
    utils::print_status("Receiving recording...");
    if opts.output.is_empty() {
        opts.output = utils::new_record_file_name(&opts.secret)?;
    }
    let mut file = File::create(&opts.output)?;
    utils::print_status(&format!("Saving as {}", opts.output));
    io::copy(&mut qsock, &mut file)?;

    if opts.play {
        opener::open(&opts.output)?;
    }
    utils::print_success("Done!");

    Ok(())
}

fn start_recording(
    device_name: &str,
    fname: PathBuf,
    interval: u64,
) -> Result<PathBuf, anyhow::Error> {
    let host = cpal::default_host();
    // Set up the input device and stream with the default input config.
    let device = host
        .default_input_device()
        .expect("failed to find input device");

    if device_name != "default" {
        host.input_devices()?
            .find(|x| x.name().map(|y| y == *device_name).unwrap_or(false))
            .expect("failed to find input device");
    }

    utils::print_verbose(&format!("Input device: {:?}", device_name));
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    utils::print_verbose(&format!("Channels: {:?}", config.channels()));
    utils::print_verbose(&format!("Sample Rate: {:?}", config.sample_rate()));
    utils::print_verbose(&format!("Buffer Size: {:?}", config.buffer_size()));
    utils::print_verbose(&format!("Sample Format: {:?}", config.sample_format()));

    // The WAV file we're recording to.
    // const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/recorded.wav");
    let spec = wav_spec_from_config(&config);
    // let writer = hound::WavWriter::new(stream, spec)?;
    let writer = hound::WavWriter::create(fname.as_path(), spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    // A flag to indicate that recording is in progress.
    utils::print_status(&format!("Recording on device: {}", device_name));

    // Run the input stream on a separate thread.
    let writer_2 = writer.clone();
    let err_fn = move |err| {
        utils::print_fatal(&format!("an error occurred on stream: {}", err));
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
            err_fn,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
            err_fn,
        )?,
        cpal::SampleFormat::U16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<u16, i16>(data, &writer_2),
            err_fn,
        )?,
    };

    // match stream.play() {
    //      Ok(()) => utils::print_status("Playing stream..."),
    //     Err(e) => utils::print_error(&e.to_string()),
    // }
    // Let recording go for roughly three seconds.

    std::thread::sleep(std::time::Duration::from_secs(interval));
    drop(stream);
    writer.lock().unwrap().take().unwrap().finalize()?;
    Ok(fname)
}

fn temp_file() -> PathBuf {
    let tmp_dir = env::temp_dir();
    let tmp_file: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();

    tmp_dir.join(format!(".{}", tmp_file))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::start_recording;
    use anyhow::Ok;

    #[test]
    fn test_recording() -> Result<(), anyhow::Error> {
        start_recording("default", PathBuf::from("/tmp/test.wav"), 5)?;
        std::thread::sleep(std::time::Duration::from_secs(5));
        Ok(())
    }
}
