// Copyright 2021 Andrew Merenbach
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::Parser;
use repeating_decimals::divide;

const DECIMAL_SEPARATOR: char = '.';

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Dividend for the quotient
    #[clap(short = 'a', long)]
    dividend: u64,

    /// Divisor for the quotient
    #[clap(short = 'b', long)]
    divisor: u64,

    /// Scale of the decimal expansion
    #[clap(short = 'c', long)]
    scale: usize,
}

fn main() {
    let args = Args::parse();

    let xs = divide(args.dividend, args.divisor, args.scale, 10u64);
    print!("{}", &xs[0]);
    if xs.len() > 1 {
        print!("{}", DECIMAL_SEPARATOR);
    }
    for i in &xs[1..] {
        print!("{}", i);
    }
}
