
## Environment variables

```cpp
SERIAL_PORT_NAME        (default: /dev/ttyUSB0)
SERIAL_PORT_BAUD_RATE   (default: 115200)
TRACING_LEVEL_FILTER    (default: 2)
```

### TRACING_LEVEL_FILTER

| Level | Value | Description |
|---:|:---:|:---|
| **Trace** | `5` | Designates very low priority, often extremely verbose, information |
| **Debug** | `4` | Designates lower priority information |
| **Info** | `3` | Designates useful information |
| **Warn** | `2` | Designates hazardous situations |
| **Error** | `1` | Designates very serious errors |
| **OFF** | `0` | Designates that trace instrumentation should be completely disabled |