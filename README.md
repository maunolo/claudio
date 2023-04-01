# Audio command line tool (claudio)

This tool allows you to send and receive audio over a network using the VBAN protocol.
VBAN is a protocol for sending audio over a network.

This tool is a work in progress. It is not yet feature complete and may not work as expected.
Please report any issues you encounter on the GitHub page.

Only supports 48kHz, 2-channels audio devices for now, 
if your using this to connect to a voicemetter application running vban in another
pc, make sure to have this setting in the stream: 48kHz, 2-channels, PCM 16 bits.

This tool is not affiliated with VBAN.

# Running examples

## Receptor
```sh
claudio vban receptor -s <YOUR_STREAM_NAME_HERE> -i <IP_TO_RECEIVE_STREAM_FROM_HERE>
```

## Emitter
```sh
claudio vban emitter -s <YOUR_STREAM_NAME_HERE> -i <IP_TO_SEND_STREAM_TO_HERE>
```

## List devices
```sh
claudio list-devices
```

## Help
```sh
vban help
```
