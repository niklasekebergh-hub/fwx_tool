pub struct Magic {
    pub filetype: &'static str,
    pub offset: usize,
    pub value: &'static [u8],
    pub validate: Option<fn(data: &[u8], off: usize) -> bool>, 
}

static MAGIC_NUMBERS: &[Magic] = &[
    Magic {
        filetype: "U-Boot uImage",
        offset: 0,
        value: &[0x27, 0x05, 0x19, 0x56],
    },
    Magic {
        filetype: "Flattened Device Tree Blob (FDTB)",
        offset: 0,
        value: &[0xd0, 0x0d, 0xfe, 0xed]
    },
    Magic {
        filetype: "FIT image",
        offset: 0,
        value: &[0xd0, 0x0d, 0xfe, 0xed]
    },
    Magic {
        filetype: "Android boot image",
        offset: 0,
        value: &[0x41, 0x4e, 0x44, 0x52, 0x4f, 0x49, 0x44, 0x21]
    },
    Magic {
        filetype: "Intel HEX",
        offset: 0,
        value: &[0x3a]
    },
    Magic {
        filetype: "Motorola S-Record",
        offset: 0,
        value: &[0x53, 0x30]
    },
    Magic {
        filetype: "gzip",
        offset: 0,
        value: &[0x1f, 0x8b, 0x08]
    },
    Magic {
        filetype: "zlib",
        offset: 0,
        value: &[0x78, 0x01]
    },
    Magic {
        filetype: "zlib",
        offset: 0,
        value: &[0x78, 0x9c]
    },
    Magic {
        filetype: "zlib",
        offset: 0,
        value: &[0x78, 0xda]
    },
    Magic {
        filetype: "bzip2",
        offset: 0,
        value: &[0x42, 0x5a, 0x68]
    },
    Magic {
        filetype: "xz",
        offset: 0,
        value: &[0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00]
    },
    Magic {
        filetype: "LZMA",
        offset: 0,
        value: &[0x5d, 0x00, 0x00, 0x80, 0x00]
    },
    Magic {
        filetype: "LZ4",
        offset: 0,
        value: &[0x04, 0x22, 0x4d, 0x18]
    },
    Magic {
        filetype: "Zstandard",
        offset: 0,
        value: &[0x28, 0xb5, 0x2f, 0xfd]
    },
    Magic {
        filetype: "SquashFS",
        offset: 0,
        value: &[0x68, 0x73, 0x71, 0x73]
    },
    Magic {
        filetype: "CRAMFS",
        offset: 0,
        value: &[0x45, 0x3d, 0xcd, 0x28]
    },
    Magic {
        filetype: "JFFS2",
        offset: 0,
        value: &[0x85, 0x19]
    },
    Magic {
        filetype: "UBI EC header",
        offset: 0,
        value: &[0x55, 0x42, 0x49, 0x23]
    },
    Magic {
        filetype: "UBIFS",
        offset: 0,
        value: &[0x55, 0x42, 0x49, 0x21]
    },
    Magic {
        filetype: "tar",
        offset: 257,
        value: &[0x75, 0x73, 0x74, 0x61, 0x72, 0x00]
    },
    Magic {
        filetype: "zip",
        offset: 0,
        value: &[0x50, 0x4b, 0x03, 0x04]
    },
    Magic {
        filetype: "zip",
        offset: 0,
        value: &[0x50, 0x4b, 0x05, 0x06]
    },
    Magic {
        filetype: "7z",
        offset: 0,
        value: &[0x37, 0x7a, 0xbc, 0xaf, 0x27, 0x1c]
    },
    Magic {
        filetype: "ELF",
        offset: 0,
        value: &[0x7f, 0x45, 0x4c, 0x46]
    },
    Magic {
        filetype: "PE/COFF",
        offset: 0,
        value: &[0x4d, 0x5a]
    },
    Magic {
        filetype: "Mach-O",
        offset: 0,
        value: &[0xfe, 0xed, 0xfa, 0xce]
    },  Magic {
        filetype: "Mach-O",
        offset: 0,
        value: &[0xce, 0xfa, 0xed, 0xfe]
    }, Magic {
        filetype: "Mach-O",
        offset: 0,
        value: &[0xfe, 0xed, 0xfa, 0xcf]
    },  Magic {
        filetype: "Mach-O",
        offset: 0,
        value: &[0xcf, 0xfa, 0xed, 0xfe]
    },
    Magic {
        filetype: "DER/ASN.1",
        offset: 0,
        value: &[0x30, 0x82]
    },
    Magic {
        filetype: "PEM",
        offset: 0,
        value: b"-----BEGIN"
    },
    Magic {
        filetype: "OpenSSH Private Key",
        offset: 0,
        value: b"-----BEGIN OPENSSH PRIVATE KEY-----"
    }
];
