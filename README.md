[![Build Status](https://travis-ci.org/azasypkin/rust-cast.svg?branch=master)](https://travis-ci.org/azasypkin/rust-cast)

# Usage
* [Documentation](https://azasypkin.github.io/rust-cast/)
* Try out [Rust Caster](https://github.com/azasypkin/rust-caster) to see this crate in action!

# DNS TXT Record description

* `md` - Model Name (e.g. "Chromecast");
* `id` - UUID without hyphens of the particular device (e.g. xx12x3x456xx789xx01xx234x56789x0);
* `fn` - Friendly Name of the device (e.g. "Living Room"); 
* `rs` - Unknown (recent share???) (e.g. "Youtube TV");
* `bs` - Uknonwn (e.g. "XX1XXX2X3456");
* `st` - Unknown (e.g. "1");
* `ca` - Unknown (e.g. "1234");
* `ic` - Icon path (e.g. "/setup/icon.png");
* `ve` - Version (e.g. "04").

# Model names

* `Chromecast` - Regular chromecast, supports video/audio;
* `Chromecast Audio` - Chromecast Audio device, supports only audio.

# Proto files

Proto files are taken from [Chromium GitHub mirror](https://github.com/chromium/chromium/tree/97ce436ecedbcd95f1375ab76d10682bef3171b0/components/cast_channel/proto).

# Useful links and sources of inspiration

* [DIAL Protocol](http://www.dial-multiscreen.org/);
* [An implementation of the Chromecast CASTV2 protocol in JS](https://github.com/thibauts/node-castv2);
* [Chromecast - steps closer to a python native api](http://www.clift.org/fred/chromecast-steps-closer-to-a-python-native-api.html);
