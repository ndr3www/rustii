pub struct Arguments {
    action: String,
    input_file_path: String,
    output_file_path: String,
    brightness: String
}

impl Arguments {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        let _binary_name = args.next();

        let action = match args.next() {
            Some(arg) => arg,
            None => return Err("{binary_name}: No action specified, run {binary_name} --help or {binary_name} -h for more info")
        };

        let (input_file_path, output_file_path, brightness) = ("".to_string(), "".to_string(), 1.0.to_string());

        match action.as_str() {
            "--help" | "-h" => return Ok(Self { action, input_file_path, output_file_path, brightness }),
            "--render" | "-r" | "--play" | "-p" => (),
            _ => return Err("{binary_name}: No such action '{action}', run {binary_name} --help or {binary_name} -h for more info")
        };

        let input_file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("{binary_name}: No input file path specified, run {binary_name} --help or {binary_name} -h for more info")
        };

        let output_file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("{binary_name}: No output file path specified, run {binary_name} --help or {binary_name} -h for more info")
        };

        let brightness = match args.next() {
            Some(arg) => arg,
            None => 1.0.to_string()
        };

        Ok(Self { action, input_file_path, output_file_path, brightness })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_help() {
        
    }
}
