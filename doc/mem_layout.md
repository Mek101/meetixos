# Maximum Indexable Address with 48 Bits

* BIN: `1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111`
* DEC: `281_474_976_710_655`
* HEX: `ffff_ffff_ffff`

Decimal:
* bytes:  `281_474_976_710_655`
* kBytes: `000_274_877_906_943`
* mBytes: `000_000_268_435_455`
* gBytes: `000_000_000_262_143`
* tBytes: `000_000_000_000_255`

# Virtual Layout

### User Address Space: `4KiB..190TiB`
* DEC: `4096..208_907_209_277_440`
* HEX: `1000..[0000_]be00_0000_0000`

### Kernel Address Space: `190TiB..255TiB`
* DEC: `211_106_232_532_989..281_474_976_710_655`
* HEX: `[0000_]be00_0000_0000..[ffff_]ffff_ffff_ffff`

### Kernel Physical Memory Mapping Area: `512GiB`
* DEC: `280_925_118_136_320..281_474_873_950_208`
* HEX: `[ffff_]ff7f_f9e0_0000..[ffff_]ffff_f9e0_0000`

### Kernel Stack Area: `2MiB`
* DEC: `281_474_873_950_208..281_474_876_047_360`
* HEX: `[ffff_]ffff_f9e0_0000..[ffff_]ffff_fea0_0000`

### Kernel Code + Data + ROData:  `95MiB`
* DEC: `281_474_876_047_360..281_474_976_710_655`
* HEX: `[ffff_]ffff_fa00_0000..[ffff_]ffff_ffff_ffff`
