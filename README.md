# Powreader
A small and simple tool I use to read out our (Dutch) smart power meter and to
send the data to an InfluxDB on another device. This allows me to run more 
utilities on the same Raspberry Pi without having it dedicated to just reading
out the power meter.

It is build to be cross-compilable to e.g. a Pi, but it can probably run on any
Linux device. It uses almost no CPU power and less than 3 MB of RAM.

## Options
```bash
$ ./powreader --help
```
```
Powreader 1.0
Powreader interprets the output from the digital power meter and outputs it in JSON or pushes it to an InfluxDB.

USAGE:
    powreader [OPTIONS] <SERIALPORT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --influxdb <influxdb>    Sets the full host and port to the influxdb. Data is transfered over UDP. This argument
                                 is given in the form of HOST:PORT. If this argument is not set, data will be printed as
                                 a JSON stream.

ARGS:
    <SERIALPORT>    Sets the serial port to use
```

## How I use it
I made a systemd configuration that makes sure that this program always runs. The execution string for me is:
```bash
/home/ubuntu/powreader --influxdb=192.168.0.50:8089 /dev/ttyUSB0
```

Where the '/dev/ttyUSB0' is the serial connection to the smart power meter and the InfluxDB-part is the location of the InfluxDB database that I use to push the data to with UDP.

Then, I use Grafana to display the measurements in graphs and gauges.