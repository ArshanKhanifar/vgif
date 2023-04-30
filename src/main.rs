use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: vgif <path-to-video-file> [crop1, crop2, ...] --outfile=<output-file name>"
        );
        std::process::exit(1);
    }

    let video_file = &args[1];
    if !fs::metadata(video_file).is_ok() {
        eprintln!("Video file does not exist");
        std::process::exit(1);
    }

    let crops = &args[2..args.len()];
    let output_file = args
        .iter()
        .find(|&arg| arg.starts_with("--outfile="))
        .map_or("out", |arg| &arg[10..]);

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let output_dir = format!("output/out_{}", timestamp);

    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    for crop in crops {
        let crop_parts: Vec<&str> = crop.split(':').collect();
        let start_time = parse_time(&crop_parts[0]).expect("Invalid crop time format");
        let end_time = parse_time(&crop_parts[1]).expect("Invalid crop time format");

        let duration = end_time - start_time;
        let output_gif = format!(
            "{}/{}_{}s-{}s.gif",
            output_dir, output_file, start_time, end_time
        );

        let command = format!(
            "ffmpeg -ss {} -t {} -i {} -pix_fmt rgb24 -r 10 {}",
            start_time, duration, video_file,  output_gif
        );

        let output = Command::new("bash")
            .arg("-c")
            .arg(&command)
            .output()
            .unwrap();

        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

fn parse_sec(sec_str: &str) -> Option<u64> {
    let second_parts: Vec<&str> = sec_str.split("s").collect();
    return second_parts[0].parse::<u64>().ok();
}

fn parse_time(time: &str) -> Option<u64> {
    let mut mins = 0;
    let mut secs = 0;

    let parts: Vec<&str> = time.trim().split(|c| c == 'm').collect();
    if parts.len() == 1 {
        return parse_sec(parts[0]);
    } else if parts.len() == 2 {
        if !parts[0].is_empty() {
            mins = parts[0].parse::<u64>().ok()?;
        }
        if !parts[1].is_empty() {
            secs = match parse_sec(parts[1]) {
                Some(s) => s,
                None => return None,
            };
        }
    } else {
        return None;
    }

    Some(mins * 60 + secs)
}

#[test]
fn test_parse_time() {
    assert_eq!(parse_time("10m2s"), Some(602));
    assert_eq!(parse_time("1m"), Some(60));
    assert_eq!(parse_time("10s"), Some(10));
    assert_eq!(parse_time("23"), Some(23));
    assert_eq!(parse_time("m10s"), Some(10));
    assert_eq!(parse_time("1m10"), Some(70));
    assert_eq!(parse_time("10m:"), None);
    assert_eq!(parse_time(":"), None);
    assert_eq!(parse_time(""), None);
}
