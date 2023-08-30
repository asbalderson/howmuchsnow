# How Much Snow

A simple command line tool for querying [how much will it snow](https://howmuchwillitsnow.com/) and reporting the inches of snow forecast for the next 7 days. 

## Usage

First build the binary using `cargo` with the command

`cargo build`

which will output a testing binary at `./target/debug/how_much_snow`

Running will yield the following output:

```
$ ./target/debug/how_much_snow --help
how-much-snow 0.1.0
Alex Balderson
Show how much snow will fall over the next 7 days for a US City

USAGE:
    how_much_snow [ARGS]

ARGS:
    <city>     
    <state>    

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

 ./target/debug/how_much_snow estes-park co
2023-08-29 - 0.000 inches
2023-08-30 - 0.000 inches
2023-08-31 - 0.000 inches
2023-09-01 - 0.000 inches
2023-09-02 - 0.000 inches
2023-09-03 - 0.000 inches
2023-09-04 - 0.000 inches
2023-09-05 - 0.000 inches
Total inches - 0.000
```

## Why

The year I purchased my house (a corner lot) we got a bunch of big heavy snows. I found myself wondering both how much snow I could expect to shovel in the coming week and how much snow i actually ended up shoveling that day. I came across [how much will it snow](https://howmuchwillitsnow.com/) to get a good reliable quantities and forecasts and wondered if I could just write a CLI tool to query and post the information. The answer was "yes" and here we are.

## TODO
* ~~Add a Readme~~
* Add makefile for building/installing
* Add a function to hyphenate the city name
* Add a function to convert a full state to it's two character abbreviation
* Look up a city and state from the zip code
* Fix help messages for arguments
* Add tests
* Add documentation 
