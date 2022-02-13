use std::time::{SystemTime, UNIX_EPOCH};use std::net::TcpStream;use std::io::Read;fn internet_time() ->f64 {let mut stream = TcpStream::connect('time.nist.gov:13').unwrap();let mut buffer = String::new();stream.read_to_string(&mut buffer).unwrap();buffer = buffer.trim().to_string();let days = buffer.split(' ').nth(0).unwrap().parse::<usize>().unwrap() - 40587;let hours = buffer.split(' ').nth(2).unwrap().split(':').nth(0).unwrap().parse::<usize>().unwrap();let mins = buffer.split(' ').nth(2).unwrap().split(':').nth(1).unwrap().parse::<usize>().unwrap();let seconds = buffer.split(' ').nth(2).unwrap().split(':').nth(2).unwrap().parse::<usize>().unwrap();let miliseconds = buffer.split(' ').nth(6).unwrap().parse::<f32>().unwrap().floor() as usize;let unix_time_stamp = days*24*60*60*1000+hours*60*60*1000+mins*60*1000+seconds*1000+miliseconds;return unix_time_stamp as f64;}fn time()->f64{let start = SystemTime::now();start.duration_since(UNIX_EPOCH).expect('Time went backwards').as_millis() as f64}fn input()->String{let mut line = String::new();std::io::stdin().read_line(&mut line).unwrap();return line.trim().to_string();}fn main(){}