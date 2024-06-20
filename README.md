<p align="left">
  <a href="https://crates.io/crates/rustii">
    <img alt="Crates.io Version" src="https://img.shields.io/crates/v/rustii" />
  </a>
  <img alt="Crates.io Downloads" src=https://img.shields.io/crates/d/rustii />
</p>

# Installation

## Through cargo
`cargo install rustii`

## Manually

### Linux
Just download the appropriate binary from [latest release](https://github.com/ndr3www/rustii/releases/latest) and copy it to a directory specified in your `$PATH`, for example:

`sudo cp rustii /usr/bin`

### Windows
1. Download **rustii.exe** from [latest release](https://github.com/ndr3www/rustii/releases/latest)
2. Create a folder named **rustii** anywhere you like
3. Copy **rustii.exe** to the newly created folder
4. Type `environment` in your search bar and hit `Enter`
5. Click the `Environment Variables...` button
6. Select **Path** variable and click `Edit...`
7. Click `New` and then `Browse...`, then find and select the **rustii** folder
8. Click `OK` and you're done!
# Usage
Basically `rustii` has 2 commands:
- `render` - generates ASCII art from a specified image
- `open` - prints specified ASCII art to the terminal/console

`rustii` compresses generated ASCII art file by default, so if you want to get the raw ASCII art, you can simply redirect the output of the `open` command, for example: `rustii open ascii.txt > raw_ascii.txt`

Type `rustii --help` or `rustii -h` to get more details

# Examples
`rustii render avatar.jpg --output ascii.txt --scale 0.22 0.09 --contrast 10`
![image](https://github.com/ndr3www/rustii/assets/164680506/75be086c-67c5-4e84-a3f3-3c65d4af80ec)

`rustii render catedral.jpg --output ascii.txt --scale 0.13 0.053`
![image(1)](https://github.com/ndr3www/rustii/assets/164680506/26d32faf-1748-4c36-b6f7-525d30b38fe2)

`rustii render woman.jpg --output ascii.txt --scale 0.09 0.032 --contrast -5`
![image(2)](https://github.com/ndr3www/rustii/assets/164680506/4c7d2e76-3536-4e92-9e4d-98cc1d1a502a)

