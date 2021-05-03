use crate::super_enum;

super_enum! {
    enum Rtype {
        Type0 => (0, "TYPE0"),
        A => (1, "A"),
        Ns => (2, "NS"),
        Md => (3, "MD"),
        Mf => (4, "MF"),
        Cname => (5, "CNAME"),
        Soa => (6, "SOA"),
        Mb => ( 7, "M"),
        Mg => (8, "MG"),
        Mr => (9, "MR"),
        Null => ( 10, "NULL"),
        Wks => (11, "WKS"),
        Ptr => (12, "PTR"),
        Hinfo => (13, "HINFO"),
        Minfo => (14, "MINFO"),
        Mx => (15, "MX"),
        Txt => (16, "TXT"),
        Rp => (17, "RP"),
        Afsdb => (18, "AFSD"),
        X25 => (19, "X25"),
        Isdn => (20, "ISDN"),
        Rt => (21, "RT"),
        Nsap => (22, "NSAP"),
        Nsapptr => (23, "NSAPPTR"),
        Sig => (24, "SIG"),
        Key => (25, "KEY"),
        Px => (26, "PX"),
        Gpos => (27, "GPOS"),
        Aaaa => ( 28, "AAAA"),
        Loc => (29, "LOC"),
        Nxt => (30, "NXT"),
        Eid => (31, "EID"),
        Nimloc => (32, "NIMLOC"),
        Srv => (33, "SRV"),
        Atma => (34, "ATMA"),
        Naptr => (35, "NAPTR"),
        Kx => (36, "KX"),
        Cert => (37, "CERT"),
        A6 => (38, "A6"),
        Dname => (39, "DNAME"),
        Sink => (40, "SINK"),
        Opt => (41, "OPT"),
        Apl => (42, "APL"),
        Ds => (43, "DS"),
        Sshfp => (44, "SSHFP"),
        Ipseckey => (45, "IPSECKEY"),
        Rrsig => (46, "RRSIG"),
        Nsec => (47, "NSEC"),
        Dnskey => (48, "DNSKEY"),
        Dhcid => (49, "DHCID"),
        Nsec3 => (50, "NSEC3"),
        Nsec3param => (51, "NSEC3PARAM"),
        Tlsa => (52, "TLSA"),
        Smimea => (53, "SMIMEA"),
        Hip => (55, "HIP"),
        Ninfo => (56, "NINFO"),
        Rkey => (57, "RKEY"),
        Talink => (58, "TALINK"),
        Cds => (59, "CDS"),
        Cdnskey => (60, "CDNSKEY"),
        Openpgpkey => (61, "OPENPGPKEY"),
        Csync => (62, "CSYNC"),
        Zonemd => (63, "ZONEMD"),
        Type65 => (65, "TYPE65"),
        Spf => (99, "SPF"),
        Uinfo => (100, "UINFO"),
        Uid => (101, "UID"),
        Gid => (102, "GID"),
        Unspec => (103, "UNSPEC"),
        Nid => (104, "NID"),
        L32 => (105, "L32"),
        L64 => (106, "L64"),
        Lp => (107, "LP"),
        Eui48 => (108, "EUI48"),
        Eui64 => (109, "EUI64"),
        Tkey => (249, "TKEY"),
        Tsig => (250, "TSIG"),
        Ixfr => (251, "IXFR"),
        Axfr => (252, "AXFR"),
        Mailb => (253, "MAIL"),
        Maila => (254, "MAILA"),
        Any => (255, "ANY"),
        Uri => (256, "URI"),
        Caa => (257, "CAA"),
        Avc => (258, "AVC"),
        Doa => (259, "DOA"),
        Ta => (32768, "TA"),
        Dlv => (32769, "DLV"),
    }
}
