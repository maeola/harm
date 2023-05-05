use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args
{
	#[arg(short, long, default_value_t = 0.75)]
	p: f64,
	#[arg(short, long, default_value_t = 1.0)]
	k: f64,
	b: Option<f64>,
}

fn slope(b: f64, p: f64, k: f64, x: f64) -> f64
{
	let h = k - p - p * k;
	let m = b + h * x;
	-(b * b + (k - 1.0) * b * x - k * x * x) * h + ((k - 1.0) * b - 2.0 * k * x) * m
}

fn search(l: f64, r: f64, b: f64, k: f64, p: f64) -> f64
{
	if slope(b, p, k, l) <= 0.0 {
		return l;
	}
	if slope(b, p, k, r) >= 0.0 {
		return r;
	}

	let m = (l + r) * 0.5;
	if r / l <= 1.0 + f64::EPSILON || r - l <= 2.0 * f64::EPSILON {
		return m;
	}

	if slope(b, p, k, m) < 0.0 {
		return search(l, m, b, k, p);
	}

	search(m, r, b, k, p)
}

fn main() -> Result<(), Box<dyn Error>>
{
	let args = Args::parse();
	let (mut p, mut k) = (args.p, args.k);
	if let Some(b) = args.b {
		println!("{}", search(0.0, b, b, k, p));
		return Ok(());
	}
	println!("(probability of success) p = {}", p);
	println!("(win/loss ratio)         k = {}", k);
	println!();
	let mut line: String;
	let mut rl = rustyline::DefaultEditor::new()?;
	loop {
		line = rl.readline("\x1b[31m>\x1b[0m ")?;
		rl.add_history_entry(line.as_str())?;
		let args: Vec<&str> = line.trim().split_whitespace().collect();
		match args.len() {
			1 => {
				let b: f64 = match args[0].parse() {
					Ok(x) => x,
					Err(_) => continue,
				};
				println!("p = {}", p);
				println!("k = {}", k);
				println!("b = {}", b);
				println!("x = {}", search(0.0, b, b, k, p));
				println!();
			}
			2 => {
				let w: f64 = match args[1].parse() {
					Ok(x) => x,
					Err(_) => continue,
				};

				match args[0] {
					"p" => p = w,
					"k" => k = w,
					_ => continue,
				};
				println!("p = {}", p);
				println!("k = {}", k);
				println!();
			}
			_ => continue,
		}
	}
}
