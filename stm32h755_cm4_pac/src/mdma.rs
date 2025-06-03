#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mdma_gisr0: MdmaGisr0,
    _reserved1: [u8; 0x3c],
    mdma_c0isr: MdmaC0isr,
    mdma_c0ifcr: MdmaC0ifcr,
    mdma_c0esr: MdmaC0esr,
    mdma_c0cr: MdmaC0cr,
    mdma_c0tcr: MdmaC0tcr,
    mdma_c0bndtr: MdmaC0bndtr,
    mdma_c0sar: MdmaC0sar,
    mdma_c0dar: MdmaC0dar,
    mdma_c0brur: MdmaC0brur,
    mdma_c0lar: MdmaC0lar,
    mdma_c0tbr: MdmaC0tbr,
    _reserved12: [u8; 0x04],
    mdma_c0mar: MdmaC0mar,
    mdma_c0mdr: MdmaC0mdr,
    _reserved14: [u8; 0x08],
    mdma_c1isr: MdmaC1isr,
    mdma_c1ifcr: MdmaC1ifcr,
    mdma_c1esr: MdmaC1esr,
    mdma_c1cr: MdmaC1cr,
    mdma_c1tcr: MdmaC1tcr,
    mdma_c1bndtr: MdmaC1bndtr,
    mdma_c1sar: MdmaC1sar,
    mdma_c1dar: MdmaC1dar,
    mdma_c1brur: MdmaC1brur,
    mdma_c1lar: MdmaC1lar,
    mdma_c1tbr: MdmaC1tbr,
    _reserved25: [u8; 0x04],
    mdma_c1mar: MdmaC1mar,
    mdma_c1mdr: MdmaC1mdr,
    _reserved27: [u8; 0x08],
    mdma_c2isr: MdmaC2isr,
    mdma_c2ifcr: MdmaC2ifcr,
    mdma_c2esr: MdmaC2esr,
    mdma_c2cr: MdmaC2cr,
    mdma_c2tcr: MdmaC2tcr,
    mdma_c2bndtr: MdmaC2bndtr,
    mdma_c2sar: MdmaC2sar,
    mdma_c2dar: MdmaC2dar,
    mdma_c2brur: MdmaC2brur,
    mdma_c2lar: MdmaC2lar,
    mdma_c2tbr: MdmaC2tbr,
    _reserved38: [u8; 0x04],
    mdma_c2mar: MdmaC2mar,
    mdma_c2mdr: MdmaC2mdr,
    _reserved40: [u8; 0x08],
    mdma_c3isr: MdmaC3isr,
    mdma_c3ifcr: MdmaC3ifcr,
    mdma_c3esr: MdmaC3esr,
    mdma_c3cr: MdmaC3cr,
    mdma_c3tcr: MdmaC3tcr,
    mdma_c3bndtr: MdmaC3bndtr,
    mdma_c3sar: MdmaC3sar,
    mdma_c3dar: MdmaC3dar,
    mdma_c3brur: MdmaC3brur,
    mdma_c3lar: MdmaC3lar,
    mdma_c3tbr: MdmaC3tbr,
    _reserved51: [u8; 0x04],
    mdma_c3mar: MdmaC3mar,
    mdma_c3mdr: MdmaC3mdr,
    _reserved53: [u8; 0x08],
    mdma_c4isr: MdmaC4isr,
    mdma_c4ifcr: MdmaC4ifcr,
    mdma_c4esr: MdmaC4esr,
    mdma_c4cr: MdmaC4cr,
    mdma_c4tcr: MdmaC4tcr,
    mdma_c4bndtr: MdmaC4bndtr,
    mdma_c4sar: MdmaC4sar,
    mdma_c4dar: MdmaC4dar,
    mdma_c4brur: MdmaC4brur,
    mdma_c4lar: MdmaC4lar,
    mdma_c4tbr: MdmaC4tbr,
    _reserved64: [u8; 0x04],
    mdma_c4mar: MdmaC4mar,
    mdma_c4mdr: MdmaC4mdr,
    _reserved66: [u8; 0x08],
    mdma_c5isr: MdmaC5isr,
    mdma_c5ifcr: MdmaC5ifcr,
    mdma_c5esr: MdmaC5esr,
    mdma_c5cr: MdmaC5cr,
    mdma_c5tcr: MdmaC5tcr,
    mdma_c5bndtr: MdmaC5bndtr,
    mdma_c5sar: MdmaC5sar,
    mdma_c5dar: MdmaC5dar,
    mdma_c5brur: MdmaC5brur,
    mdma_c5lar: MdmaC5lar,
    mdma_c5tbr: MdmaC5tbr,
    _reserved77: [u8; 0x04],
    mdma_c5mar: MdmaC5mar,
    mdma_c5mdr: MdmaC5mdr,
    _reserved79: [u8; 0x08],
    mdma_c6isr: MdmaC6isr,
    mdma_c6ifcr: MdmaC6ifcr,
    mdma_c6esr: MdmaC6esr,
    mdma_c6cr: MdmaC6cr,
    mdma_c6tcr: MdmaC6tcr,
    mdma_c6bndtr: MdmaC6bndtr,
    mdma_c6sar: MdmaC6sar,
    mdma_c6dar: MdmaC6dar,
    mdma_c6brur: MdmaC6brur,
    mdma_c6lar: MdmaC6lar,
    mdma_c6tbr: MdmaC6tbr,
    _reserved90: [u8; 0x04],
    mdma_c6mar: MdmaC6mar,
    mdma_c6mdr: MdmaC6mdr,
    _reserved92: [u8; 0x08],
    mdma_c7isr: MdmaC7isr,
    mdma_c7ifcr: MdmaC7ifcr,
    mdma_c7esr: MdmaC7esr,
    mdma_c7cr: MdmaC7cr,
    mdma_c7tcr: MdmaC7tcr,
    mdma_c7bndtr: MdmaC7bndtr,
    mdma_c7sar: MdmaC7sar,
    mdma_c7dar: MdmaC7dar,
    mdma_c7brur: MdmaC7brur,
    mdma_c7lar: MdmaC7lar,
    mdma_c7tbr: MdmaC7tbr,
    _reserved103: [u8; 0x04],
    mdma_c7mar: MdmaC7mar,
    mdma_c7mdr: MdmaC7mdr,
    _reserved105: [u8; 0x08],
    mdma_c8isr: MdmaC8isr,
    mdma_c8ifcr: MdmaC8ifcr,
    mdma_c8esr: MdmaC8esr,
    mdma_c8cr: MdmaC8cr,
    mdma_c8tcr: MdmaC8tcr,
    mdma_c8bndtr: MdmaC8bndtr,
    mdma_c8sar: MdmaC8sar,
    mdma_c8dar: MdmaC8dar,
    mdma_c8brur: MdmaC8brur,
    mdma_c8lar: MdmaC8lar,
    mdma_c8tbr: MdmaC8tbr,
    _reserved116: [u8; 0x04],
    mdma_c8mar: MdmaC8mar,
    mdma_c8mdr: MdmaC8mdr,
    _reserved118: [u8; 0x08],
    mdma_c9isr: MdmaC9isr,
    mdma_c9ifcr: MdmaC9ifcr,
    mdma_c9esr: MdmaC9esr,
    mdma_c9cr: MdmaC9cr,
    mdma_c9tcr: MdmaC9tcr,
    mdma_c9bndtr: MdmaC9bndtr,
    mdma_c9sar: MdmaC9sar,
    mdma_c9dar: MdmaC9dar,
    mdma_c9brur: MdmaC9brur,
    mdma_c9lar: MdmaC9lar,
    mdma_c9tbr: MdmaC9tbr,
    _reserved129: [u8; 0x04],
    mdma_c9mar: MdmaC9mar,
    mdma_c9mdr: MdmaC9mdr,
    _reserved131: [u8; 0x08],
    mdma_c10isr: MdmaC10isr,
    mdma_c10ifcr: MdmaC10ifcr,
    mdma_c10esr: MdmaC10esr,
    mdma_c10cr: MdmaC10cr,
    mdma_c10tcr: MdmaC10tcr,
    mdma_c10bndtr: MdmaC10bndtr,
    mdma_c10sar: MdmaC10sar,
    mdma_c10dar: MdmaC10dar,
    mdma_c10brur: MdmaC10brur,
    mdma_c10lar: MdmaC10lar,
    mdma_c10tbr: MdmaC10tbr,
    _reserved142: [u8; 0x04],
    mdma_c10mar: MdmaC10mar,
    mdma_c10mdr: MdmaC10mdr,
    _reserved144: [u8; 0x08],
    mdma_c11isr: MdmaC11isr,
    mdma_c11ifcr: MdmaC11ifcr,
    mdma_c11esr: MdmaC11esr,
    mdma_c11cr: MdmaC11cr,
    mdma_c11tcr: MdmaC11tcr,
    mdma_c11bndtr: MdmaC11bndtr,
    mdma_c11sar: MdmaC11sar,
    mdma_c11dar: MdmaC11dar,
    mdma_c11brur: MdmaC11brur,
    mdma_c11lar: MdmaC11lar,
    mdma_c11tbr: MdmaC11tbr,
    _reserved155: [u8; 0x04],
    mdma_c11mar: MdmaC11mar,
    mdma_c11mdr: MdmaC11mdr,
    _reserved157: [u8; 0x08],
    mdma_c12isr: MdmaC12isr,
    mdma_c12ifcr: MdmaC12ifcr,
    mdma_c12esr: MdmaC12esr,
    mdma_c12cr: MdmaC12cr,
    mdma_c12tcr: MdmaC12tcr,
    mdma_c12bndtr: MdmaC12bndtr,
    mdma_c12sar: MdmaC12sar,
    mdma_c12dar: MdmaC12dar,
    mdma_c12brur: MdmaC12brur,
    mdma_c12lar: MdmaC12lar,
    mdma_c12tbr: MdmaC12tbr,
    _reserved168: [u8; 0x04],
    mdma_c12mar: MdmaC12mar,
    mdma_c12mdr: MdmaC12mdr,
    _reserved170: [u8; 0x08],
    mdma_c13isr: MdmaC13isr,
    mdma_c13ifcr: MdmaC13ifcr,
    mdma_c13esr: MdmaC13esr,
    mdma_c13cr: MdmaC13cr,
    mdma_c13tcr: MdmaC13tcr,
    mdma_c13bndtr: MdmaC13bndtr,
    mdma_c13sar: MdmaC13sar,
    mdma_c13dar: MdmaC13dar,
    mdma_c13brur: MdmaC13brur,
    mdma_c13lar: MdmaC13lar,
    mdma_c13tbr: MdmaC13tbr,
    _reserved181: [u8; 0x04],
    mdma_c13mar: MdmaC13mar,
    mdma_c13mdr: MdmaC13mdr,
    _reserved183: [u8; 0x08],
    mdma_c14isr: MdmaC14isr,
    mdma_c14ifcr: MdmaC14ifcr,
    mdma_c14esr: MdmaC14esr,
    mdma_c14cr: MdmaC14cr,
    mdma_c14tcr: MdmaC14tcr,
    mdma_c14bndtr: MdmaC14bndtr,
    mdma_c14sar: MdmaC14sar,
    mdma_c14dar: MdmaC14dar,
    mdma_c14brur: MdmaC14brur,
    mdma_c14lar: MdmaC14lar,
    mdma_c14tbr: MdmaC14tbr,
    _reserved194: [u8; 0x04],
    mdma_c14mar: MdmaC14mar,
    mdma_c14mdr: MdmaC14mdr,
    _reserved196: [u8; 0x08],
    mdma_c15isr: MdmaC15isr,
    mdma_c15ifcr: MdmaC15ifcr,
    mdma_c15esr: MdmaC15esr,
    mdma_c15cr: MdmaC15cr,
    mdma_c15tcr: MdmaC15tcr,
    mdma_c15bndtr: MdmaC15bndtr,
    mdma_c15sar: MdmaC15sar,
    mdma_c15dar: MdmaC15dar,
    mdma_c15brur: MdmaC15brur,
    mdma_c15lar: MdmaC15lar,
    mdma_c15tbr: MdmaC15tbr,
    _reserved207: [u8; 0x04],
    mdma_c15mar: MdmaC15mar,
    mdma_c15mdr: MdmaC15mdr,
}
impl RegisterBlock {
    #[doc = "0x00 - MDMA Global Interrupt/Status Register"]
    #[inline(always)]
    pub const fn mdma_gisr0(&self) -> &MdmaGisr0 {
        &self.mdma_gisr0
    }
    #[doc = "0x40 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c0isr(&self) -> &MdmaC0isr {
        &self.mdma_c0isr
    }
    #[doc = "0x44 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c0ifcr(&self) -> &MdmaC0ifcr {
        &self.mdma_c0ifcr
    }
    #[doc = "0x48 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c0esr(&self) -> &MdmaC0esr {
        &self.mdma_c0esr
    }
    #[doc = "0x4c - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c0cr(&self) -> &MdmaC0cr {
        &self.mdma_c0cr
    }
    #[doc = "0x50 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c0tcr(&self) -> &MdmaC0tcr {
        &self.mdma_c0tcr
    }
    #[doc = "0x54 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c0bndtr(&self) -> &MdmaC0bndtr {
        &self.mdma_c0bndtr
    }
    #[doc = "0x58 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c0sar(&self) -> &MdmaC0sar {
        &self.mdma_c0sar
    }
    #[doc = "0x5c - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c0dar(&self) -> &MdmaC0dar {
        &self.mdma_c0dar
    }
    #[doc = "0x60 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c0brur(&self) -> &MdmaC0brur {
        &self.mdma_c0brur
    }
    #[doc = "0x64 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c0lar(&self) -> &MdmaC0lar {
        &self.mdma_c0lar
    }
    #[doc = "0x68 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c0tbr(&self) -> &MdmaC0tbr {
        &self.mdma_c0tbr
    }
    #[doc = "0x70 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c0mar(&self) -> &MdmaC0mar {
        &self.mdma_c0mar
    }
    #[doc = "0x74 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c0mdr(&self) -> &MdmaC0mdr {
        &self.mdma_c0mdr
    }
    #[doc = "0x80 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c1isr(&self) -> &MdmaC1isr {
        &self.mdma_c1isr
    }
    #[doc = "0x84 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c1ifcr(&self) -> &MdmaC1ifcr {
        &self.mdma_c1ifcr
    }
    #[doc = "0x88 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c1esr(&self) -> &MdmaC1esr {
        &self.mdma_c1esr
    }
    #[doc = "0x8c - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c1cr(&self) -> &MdmaC1cr {
        &self.mdma_c1cr
    }
    #[doc = "0x90 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c1tcr(&self) -> &MdmaC1tcr {
        &self.mdma_c1tcr
    }
    #[doc = "0x94 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c1bndtr(&self) -> &MdmaC1bndtr {
        &self.mdma_c1bndtr
    }
    #[doc = "0x98 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c1sar(&self) -> &MdmaC1sar {
        &self.mdma_c1sar
    }
    #[doc = "0x9c - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c1dar(&self) -> &MdmaC1dar {
        &self.mdma_c1dar
    }
    #[doc = "0xa0 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c1brur(&self) -> &MdmaC1brur {
        &self.mdma_c1brur
    }
    #[doc = "0xa4 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c1lar(&self) -> &MdmaC1lar {
        &self.mdma_c1lar
    }
    #[doc = "0xa8 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c1tbr(&self) -> &MdmaC1tbr {
        &self.mdma_c1tbr
    }
    #[doc = "0xb0 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c1mar(&self) -> &MdmaC1mar {
        &self.mdma_c1mar
    }
    #[doc = "0xb4 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c1mdr(&self) -> &MdmaC1mdr {
        &self.mdma_c1mdr
    }
    #[doc = "0xc0 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c2isr(&self) -> &MdmaC2isr {
        &self.mdma_c2isr
    }
    #[doc = "0xc4 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c2ifcr(&self) -> &MdmaC2ifcr {
        &self.mdma_c2ifcr
    }
    #[doc = "0xc8 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c2esr(&self) -> &MdmaC2esr {
        &self.mdma_c2esr
    }
    #[doc = "0xcc - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c2cr(&self) -> &MdmaC2cr {
        &self.mdma_c2cr
    }
    #[doc = "0xd0 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c2tcr(&self) -> &MdmaC2tcr {
        &self.mdma_c2tcr
    }
    #[doc = "0xd4 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c2bndtr(&self) -> &MdmaC2bndtr {
        &self.mdma_c2bndtr
    }
    #[doc = "0xd8 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c2sar(&self) -> &MdmaC2sar {
        &self.mdma_c2sar
    }
    #[doc = "0xdc - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c2dar(&self) -> &MdmaC2dar {
        &self.mdma_c2dar
    }
    #[doc = "0xe0 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c2brur(&self) -> &MdmaC2brur {
        &self.mdma_c2brur
    }
    #[doc = "0xe4 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c2lar(&self) -> &MdmaC2lar {
        &self.mdma_c2lar
    }
    #[doc = "0xe8 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c2tbr(&self) -> &MdmaC2tbr {
        &self.mdma_c2tbr
    }
    #[doc = "0xf0 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c2mar(&self) -> &MdmaC2mar {
        &self.mdma_c2mar
    }
    #[doc = "0xf4 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c2mdr(&self) -> &MdmaC2mdr {
        &self.mdma_c2mdr
    }
    #[doc = "0x100 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c3isr(&self) -> &MdmaC3isr {
        &self.mdma_c3isr
    }
    #[doc = "0x104 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c3ifcr(&self) -> &MdmaC3ifcr {
        &self.mdma_c3ifcr
    }
    #[doc = "0x108 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c3esr(&self) -> &MdmaC3esr {
        &self.mdma_c3esr
    }
    #[doc = "0x10c - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c3cr(&self) -> &MdmaC3cr {
        &self.mdma_c3cr
    }
    #[doc = "0x110 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c3tcr(&self) -> &MdmaC3tcr {
        &self.mdma_c3tcr
    }
    #[doc = "0x114 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c3bndtr(&self) -> &MdmaC3bndtr {
        &self.mdma_c3bndtr
    }
    #[doc = "0x118 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c3sar(&self) -> &MdmaC3sar {
        &self.mdma_c3sar
    }
    #[doc = "0x11c - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c3dar(&self) -> &MdmaC3dar {
        &self.mdma_c3dar
    }
    #[doc = "0x120 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c3brur(&self) -> &MdmaC3brur {
        &self.mdma_c3brur
    }
    #[doc = "0x124 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c3lar(&self) -> &MdmaC3lar {
        &self.mdma_c3lar
    }
    #[doc = "0x128 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c3tbr(&self) -> &MdmaC3tbr {
        &self.mdma_c3tbr
    }
    #[doc = "0x130 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c3mar(&self) -> &MdmaC3mar {
        &self.mdma_c3mar
    }
    #[doc = "0x134 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c3mdr(&self) -> &MdmaC3mdr {
        &self.mdma_c3mdr
    }
    #[doc = "0x140 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c4isr(&self) -> &MdmaC4isr {
        &self.mdma_c4isr
    }
    #[doc = "0x144 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c4ifcr(&self) -> &MdmaC4ifcr {
        &self.mdma_c4ifcr
    }
    #[doc = "0x148 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c4esr(&self) -> &MdmaC4esr {
        &self.mdma_c4esr
    }
    #[doc = "0x14c - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c4cr(&self) -> &MdmaC4cr {
        &self.mdma_c4cr
    }
    #[doc = "0x150 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c4tcr(&self) -> &MdmaC4tcr {
        &self.mdma_c4tcr
    }
    #[doc = "0x154 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c4bndtr(&self) -> &MdmaC4bndtr {
        &self.mdma_c4bndtr
    }
    #[doc = "0x158 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c4sar(&self) -> &MdmaC4sar {
        &self.mdma_c4sar
    }
    #[doc = "0x15c - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c4dar(&self) -> &MdmaC4dar {
        &self.mdma_c4dar
    }
    #[doc = "0x160 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c4brur(&self) -> &MdmaC4brur {
        &self.mdma_c4brur
    }
    #[doc = "0x164 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c4lar(&self) -> &MdmaC4lar {
        &self.mdma_c4lar
    }
    #[doc = "0x168 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c4tbr(&self) -> &MdmaC4tbr {
        &self.mdma_c4tbr
    }
    #[doc = "0x170 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c4mar(&self) -> &MdmaC4mar {
        &self.mdma_c4mar
    }
    #[doc = "0x174 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c4mdr(&self) -> &MdmaC4mdr {
        &self.mdma_c4mdr
    }
    #[doc = "0x180 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c5isr(&self) -> &MdmaC5isr {
        &self.mdma_c5isr
    }
    #[doc = "0x184 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c5ifcr(&self) -> &MdmaC5ifcr {
        &self.mdma_c5ifcr
    }
    #[doc = "0x188 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c5esr(&self) -> &MdmaC5esr {
        &self.mdma_c5esr
    }
    #[doc = "0x18c - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c5cr(&self) -> &MdmaC5cr {
        &self.mdma_c5cr
    }
    #[doc = "0x190 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c5tcr(&self) -> &MdmaC5tcr {
        &self.mdma_c5tcr
    }
    #[doc = "0x194 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c5bndtr(&self) -> &MdmaC5bndtr {
        &self.mdma_c5bndtr
    }
    #[doc = "0x198 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c5sar(&self) -> &MdmaC5sar {
        &self.mdma_c5sar
    }
    #[doc = "0x19c - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c5dar(&self) -> &MdmaC5dar {
        &self.mdma_c5dar
    }
    #[doc = "0x1a0 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c5brur(&self) -> &MdmaC5brur {
        &self.mdma_c5brur
    }
    #[doc = "0x1a4 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c5lar(&self) -> &MdmaC5lar {
        &self.mdma_c5lar
    }
    #[doc = "0x1a8 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c5tbr(&self) -> &MdmaC5tbr {
        &self.mdma_c5tbr
    }
    #[doc = "0x1b0 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c5mar(&self) -> &MdmaC5mar {
        &self.mdma_c5mar
    }
    #[doc = "0x1b4 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c5mdr(&self) -> &MdmaC5mdr {
        &self.mdma_c5mdr
    }
    #[doc = "0x1c0 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c6isr(&self) -> &MdmaC6isr {
        &self.mdma_c6isr
    }
    #[doc = "0x1c4 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c6ifcr(&self) -> &MdmaC6ifcr {
        &self.mdma_c6ifcr
    }
    #[doc = "0x1c8 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c6esr(&self) -> &MdmaC6esr {
        &self.mdma_c6esr
    }
    #[doc = "0x1cc - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c6cr(&self) -> &MdmaC6cr {
        &self.mdma_c6cr
    }
    #[doc = "0x1d0 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c6tcr(&self) -> &MdmaC6tcr {
        &self.mdma_c6tcr
    }
    #[doc = "0x1d4 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c6bndtr(&self) -> &MdmaC6bndtr {
        &self.mdma_c6bndtr
    }
    #[doc = "0x1d8 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c6sar(&self) -> &MdmaC6sar {
        &self.mdma_c6sar
    }
    #[doc = "0x1dc - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c6dar(&self) -> &MdmaC6dar {
        &self.mdma_c6dar
    }
    #[doc = "0x1e0 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c6brur(&self) -> &MdmaC6brur {
        &self.mdma_c6brur
    }
    #[doc = "0x1e4 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c6lar(&self) -> &MdmaC6lar {
        &self.mdma_c6lar
    }
    #[doc = "0x1e8 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c6tbr(&self) -> &MdmaC6tbr {
        &self.mdma_c6tbr
    }
    #[doc = "0x1f0 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c6mar(&self) -> &MdmaC6mar {
        &self.mdma_c6mar
    }
    #[doc = "0x1f4 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c6mdr(&self) -> &MdmaC6mdr {
        &self.mdma_c6mdr
    }
    #[doc = "0x200 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c7isr(&self) -> &MdmaC7isr {
        &self.mdma_c7isr
    }
    #[doc = "0x204 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c7ifcr(&self) -> &MdmaC7ifcr {
        &self.mdma_c7ifcr
    }
    #[doc = "0x208 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c7esr(&self) -> &MdmaC7esr {
        &self.mdma_c7esr
    }
    #[doc = "0x20c - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c7cr(&self) -> &MdmaC7cr {
        &self.mdma_c7cr
    }
    #[doc = "0x210 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c7tcr(&self) -> &MdmaC7tcr {
        &self.mdma_c7tcr
    }
    #[doc = "0x214 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c7bndtr(&self) -> &MdmaC7bndtr {
        &self.mdma_c7bndtr
    }
    #[doc = "0x218 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c7sar(&self) -> &MdmaC7sar {
        &self.mdma_c7sar
    }
    #[doc = "0x21c - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c7dar(&self) -> &MdmaC7dar {
        &self.mdma_c7dar
    }
    #[doc = "0x220 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c7brur(&self) -> &MdmaC7brur {
        &self.mdma_c7brur
    }
    #[doc = "0x224 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c7lar(&self) -> &MdmaC7lar {
        &self.mdma_c7lar
    }
    #[doc = "0x228 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c7tbr(&self) -> &MdmaC7tbr {
        &self.mdma_c7tbr
    }
    #[doc = "0x230 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c7mar(&self) -> &MdmaC7mar {
        &self.mdma_c7mar
    }
    #[doc = "0x234 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c7mdr(&self) -> &MdmaC7mdr {
        &self.mdma_c7mdr
    }
    #[doc = "0x240 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c8isr(&self) -> &MdmaC8isr {
        &self.mdma_c8isr
    }
    #[doc = "0x244 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c8ifcr(&self) -> &MdmaC8ifcr {
        &self.mdma_c8ifcr
    }
    #[doc = "0x248 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c8esr(&self) -> &MdmaC8esr {
        &self.mdma_c8esr
    }
    #[doc = "0x24c - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c8cr(&self) -> &MdmaC8cr {
        &self.mdma_c8cr
    }
    #[doc = "0x250 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c8tcr(&self) -> &MdmaC8tcr {
        &self.mdma_c8tcr
    }
    #[doc = "0x254 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c8bndtr(&self) -> &MdmaC8bndtr {
        &self.mdma_c8bndtr
    }
    #[doc = "0x258 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c8sar(&self) -> &MdmaC8sar {
        &self.mdma_c8sar
    }
    #[doc = "0x25c - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c8dar(&self) -> &MdmaC8dar {
        &self.mdma_c8dar
    }
    #[doc = "0x260 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c8brur(&self) -> &MdmaC8brur {
        &self.mdma_c8brur
    }
    #[doc = "0x264 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c8lar(&self) -> &MdmaC8lar {
        &self.mdma_c8lar
    }
    #[doc = "0x268 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c8tbr(&self) -> &MdmaC8tbr {
        &self.mdma_c8tbr
    }
    #[doc = "0x270 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c8mar(&self) -> &MdmaC8mar {
        &self.mdma_c8mar
    }
    #[doc = "0x274 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c8mdr(&self) -> &MdmaC8mdr {
        &self.mdma_c8mdr
    }
    #[doc = "0x280 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c9isr(&self) -> &MdmaC9isr {
        &self.mdma_c9isr
    }
    #[doc = "0x284 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c9ifcr(&self) -> &MdmaC9ifcr {
        &self.mdma_c9ifcr
    }
    #[doc = "0x288 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c9esr(&self) -> &MdmaC9esr {
        &self.mdma_c9esr
    }
    #[doc = "0x28c - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c9cr(&self) -> &MdmaC9cr {
        &self.mdma_c9cr
    }
    #[doc = "0x290 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c9tcr(&self) -> &MdmaC9tcr {
        &self.mdma_c9tcr
    }
    #[doc = "0x294 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c9bndtr(&self) -> &MdmaC9bndtr {
        &self.mdma_c9bndtr
    }
    #[doc = "0x298 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c9sar(&self) -> &MdmaC9sar {
        &self.mdma_c9sar
    }
    #[doc = "0x29c - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c9dar(&self) -> &MdmaC9dar {
        &self.mdma_c9dar
    }
    #[doc = "0x2a0 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c9brur(&self) -> &MdmaC9brur {
        &self.mdma_c9brur
    }
    #[doc = "0x2a4 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c9lar(&self) -> &MdmaC9lar {
        &self.mdma_c9lar
    }
    #[doc = "0x2a8 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c9tbr(&self) -> &MdmaC9tbr {
        &self.mdma_c9tbr
    }
    #[doc = "0x2b0 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c9mar(&self) -> &MdmaC9mar {
        &self.mdma_c9mar
    }
    #[doc = "0x2b4 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c9mdr(&self) -> &MdmaC9mdr {
        &self.mdma_c9mdr
    }
    #[doc = "0x2c0 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c10isr(&self) -> &MdmaC10isr {
        &self.mdma_c10isr
    }
    #[doc = "0x2c4 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c10ifcr(&self) -> &MdmaC10ifcr {
        &self.mdma_c10ifcr
    }
    #[doc = "0x2c8 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c10esr(&self) -> &MdmaC10esr {
        &self.mdma_c10esr
    }
    #[doc = "0x2cc - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c10cr(&self) -> &MdmaC10cr {
        &self.mdma_c10cr
    }
    #[doc = "0x2d0 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c10tcr(&self) -> &MdmaC10tcr {
        &self.mdma_c10tcr
    }
    #[doc = "0x2d4 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c10bndtr(&self) -> &MdmaC10bndtr {
        &self.mdma_c10bndtr
    }
    #[doc = "0x2d8 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c10sar(&self) -> &MdmaC10sar {
        &self.mdma_c10sar
    }
    #[doc = "0x2dc - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c10dar(&self) -> &MdmaC10dar {
        &self.mdma_c10dar
    }
    #[doc = "0x2e0 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c10brur(&self) -> &MdmaC10brur {
        &self.mdma_c10brur
    }
    #[doc = "0x2e4 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c10lar(&self) -> &MdmaC10lar {
        &self.mdma_c10lar
    }
    #[doc = "0x2e8 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c10tbr(&self) -> &MdmaC10tbr {
        &self.mdma_c10tbr
    }
    #[doc = "0x2f0 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c10mar(&self) -> &MdmaC10mar {
        &self.mdma_c10mar
    }
    #[doc = "0x2f4 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c10mdr(&self) -> &MdmaC10mdr {
        &self.mdma_c10mdr
    }
    #[doc = "0x300 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c11isr(&self) -> &MdmaC11isr {
        &self.mdma_c11isr
    }
    #[doc = "0x304 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c11ifcr(&self) -> &MdmaC11ifcr {
        &self.mdma_c11ifcr
    }
    #[doc = "0x308 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c11esr(&self) -> &MdmaC11esr {
        &self.mdma_c11esr
    }
    #[doc = "0x30c - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c11cr(&self) -> &MdmaC11cr {
        &self.mdma_c11cr
    }
    #[doc = "0x310 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c11tcr(&self) -> &MdmaC11tcr {
        &self.mdma_c11tcr
    }
    #[doc = "0x314 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c11bndtr(&self) -> &MdmaC11bndtr {
        &self.mdma_c11bndtr
    }
    #[doc = "0x318 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c11sar(&self) -> &MdmaC11sar {
        &self.mdma_c11sar
    }
    #[doc = "0x31c - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c11dar(&self) -> &MdmaC11dar {
        &self.mdma_c11dar
    }
    #[doc = "0x320 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c11brur(&self) -> &MdmaC11brur {
        &self.mdma_c11brur
    }
    #[doc = "0x324 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c11lar(&self) -> &MdmaC11lar {
        &self.mdma_c11lar
    }
    #[doc = "0x328 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c11tbr(&self) -> &MdmaC11tbr {
        &self.mdma_c11tbr
    }
    #[doc = "0x330 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c11mar(&self) -> &MdmaC11mar {
        &self.mdma_c11mar
    }
    #[doc = "0x334 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c11mdr(&self) -> &MdmaC11mdr {
        &self.mdma_c11mdr
    }
    #[doc = "0x340 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c12isr(&self) -> &MdmaC12isr {
        &self.mdma_c12isr
    }
    #[doc = "0x344 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c12ifcr(&self) -> &MdmaC12ifcr {
        &self.mdma_c12ifcr
    }
    #[doc = "0x348 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c12esr(&self) -> &MdmaC12esr {
        &self.mdma_c12esr
    }
    #[doc = "0x34c - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c12cr(&self) -> &MdmaC12cr {
        &self.mdma_c12cr
    }
    #[doc = "0x350 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c12tcr(&self) -> &MdmaC12tcr {
        &self.mdma_c12tcr
    }
    #[doc = "0x354 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c12bndtr(&self) -> &MdmaC12bndtr {
        &self.mdma_c12bndtr
    }
    #[doc = "0x358 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c12sar(&self) -> &MdmaC12sar {
        &self.mdma_c12sar
    }
    #[doc = "0x35c - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c12dar(&self) -> &MdmaC12dar {
        &self.mdma_c12dar
    }
    #[doc = "0x360 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c12brur(&self) -> &MdmaC12brur {
        &self.mdma_c12brur
    }
    #[doc = "0x364 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c12lar(&self) -> &MdmaC12lar {
        &self.mdma_c12lar
    }
    #[doc = "0x368 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c12tbr(&self) -> &MdmaC12tbr {
        &self.mdma_c12tbr
    }
    #[doc = "0x370 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c12mar(&self) -> &MdmaC12mar {
        &self.mdma_c12mar
    }
    #[doc = "0x374 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c12mdr(&self) -> &MdmaC12mdr {
        &self.mdma_c12mdr
    }
    #[doc = "0x380 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c13isr(&self) -> &MdmaC13isr {
        &self.mdma_c13isr
    }
    #[doc = "0x384 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c13ifcr(&self) -> &MdmaC13ifcr {
        &self.mdma_c13ifcr
    }
    #[doc = "0x388 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c13esr(&self) -> &MdmaC13esr {
        &self.mdma_c13esr
    }
    #[doc = "0x38c - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c13cr(&self) -> &MdmaC13cr {
        &self.mdma_c13cr
    }
    #[doc = "0x390 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c13tcr(&self) -> &MdmaC13tcr {
        &self.mdma_c13tcr
    }
    #[doc = "0x394 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c13bndtr(&self) -> &MdmaC13bndtr {
        &self.mdma_c13bndtr
    }
    #[doc = "0x398 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c13sar(&self) -> &MdmaC13sar {
        &self.mdma_c13sar
    }
    #[doc = "0x39c - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c13dar(&self) -> &MdmaC13dar {
        &self.mdma_c13dar
    }
    #[doc = "0x3a0 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c13brur(&self) -> &MdmaC13brur {
        &self.mdma_c13brur
    }
    #[doc = "0x3a4 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c13lar(&self) -> &MdmaC13lar {
        &self.mdma_c13lar
    }
    #[doc = "0x3a8 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c13tbr(&self) -> &MdmaC13tbr {
        &self.mdma_c13tbr
    }
    #[doc = "0x3b0 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c13mar(&self) -> &MdmaC13mar {
        &self.mdma_c13mar
    }
    #[doc = "0x3b4 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c13mdr(&self) -> &MdmaC13mdr {
        &self.mdma_c13mdr
    }
    #[doc = "0x3c0 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c14isr(&self) -> &MdmaC14isr {
        &self.mdma_c14isr
    }
    #[doc = "0x3c4 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c14ifcr(&self) -> &MdmaC14ifcr {
        &self.mdma_c14ifcr
    }
    #[doc = "0x3c8 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c14esr(&self) -> &MdmaC14esr {
        &self.mdma_c14esr
    }
    #[doc = "0x3cc - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c14cr(&self) -> &MdmaC14cr {
        &self.mdma_c14cr
    }
    #[doc = "0x3d0 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c14tcr(&self) -> &MdmaC14tcr {
        &self.mdma_c14tcr
    }
    #[doc = "0x3d4 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c14bndtr(&self) -> &MdmaC14bndtr {
        &self.mdma_c14bndtr
    }
    #[doc = "0x3d8 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c14sar(&self) -> &MdmaC14sar {
        &self.mdma_c14sar
    }
    #[doc = "0x3dc - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c14dar(&self) -> &MdmaC14dar {
        &self.mdma_c14dar
    }
    #[doc = "0x3e0 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c14brur(&self) -> &MdmaC14brur {
        &self.mdma_c14brur
    }
    #[doc = "0x3e4 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c14lar(&self) -> &MdmaC14lar {
        &self.mdma_c14lar
    }
    #[doc = "0x3e8 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c14tbr(&self) -> &MdmaC14tbr {
        &self.mdma_c14tbr
    }
    #[doc = "0x3f0 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c14mar(&self) -> &MdmaC14mar {
        &self.mdma_c14mar
    }
    #[doc = "0x3f4 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c14mdr(&self) -> &MdmaC14mdr {
        &self.mdma_c14mdr
    }
    #[doc = "0x400 - MDMA channel x interrupt/status register"]
    #[inline(always)]
    pub const fn mdma_c15isr(&self) -> &MdmaC15isr {
        &self.mdma_c15isr
    }
    #[doc = "0x404 - MDMA channel x interrupt flag clear register"]
    #[inline(always)]
    pub const fn mdma_c15ifcr(&self) -> &MdmaC15ifcr {
        &self.mdma_c15ifcr
    }
    #[doc = "0x408 - MDMA Channel x error status register"]
    #[inline(always)]
    pub const fn mdma_c15esr(&self) -> &MdmaC15esr {
        &self.mdma_c15esr
    }
    #[doc = "0x40c - This register is used to control the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c15cr(&self) -> &MdmaC15cr {
        &self.mdma_c15cr
    }
    #[doc = "0x410 - This register is used to configure the concerned channel."]
    #[inline(always)]
    pub const fn mdma_c15tcr(&self) -> &MdmaC15tcr {
        &self.mdma_c15tcr
    }
    #[doc = "0x414 - MDMA Channel x block number of data register"]
    #[inline(always)]
    pub const fn mdma_c15bndtr(&self) -> &MdmaC15bndtr {
        &self.mdma_c15bndtr
    }
    #[doc = "0x418 - MDMA channel x source address register"]
    #[inline(always)]
    pub const fn mdma_c15sar(&self) -> &MdmaC15sar {
        &self.mdma_c15sar
    }
    #[doc = "0x41c - MDMA channel x destination address register"]
    #[inline(always)]
    pub const fn mdma_c15dar(&self) -> &MdmaC15dar {
        &self.mdma_c15dar
    }
    #[doc = "0x420 - MDMA channel x Block Repeat address Update register"]
    #[inline(always)]
    pub const fn mdma_c15brur(&self) -> &MdmaC15brur {
        &self.mdma_c15brur
    }
    #[doc = "0x424 - MDMA channel x Link Address register"]
    #[inline(always)]
    pub const fn mdma_c15lar(&self) -> &MdmaC15lar {
        &self.mdma_c15lar
    }
    #[doc = "0x428 - MDMA channel x Trigger and Bus selection Register"]
    #[inline(always)]
    pub const fn mdma_c15tbr(&self) -> &MdmaC15tbr {
        &self.mdma_c15tbr
    }
    #[doc = "0x430 - MDMA channel x Mask address register"]
    #[inline(always)]
    pub const fn mdma_c15mar(&self) -> &MdmaC15mar {
        &self.mdma_c15mar
    }
    #[doc = "0x434 - MDMA channel x Mask Data register"]
    #[inline(always)]
    pub const fn mdma_c15mdr(&self) -> &MdmaC15mdr {
        &self.mdma_c15mdr
    }
}
#[doc = "MDMA_GISR0 (r) register accessor: MDMA Global Interrupt/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_gisr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_gisr0`] module"]
#[doc(alias = "MDMA_GISR0")]
pub type MdmaGisr0 = crate::Reg<mdma_gisr0::MdmaGisr0Spec>;
#[doc = "MDMA Global Interrupt/Status Register"]
pub mod mdma_gisr0;
#[doc = "MDMA_C0ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c0isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c0isr`] module"]
#[doc(alias = "MDMA_C0ISR")]
pub type MdmaC0isr = crate::Reg<mdma_c0isr::MdmaC0isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c0isr;
#[doc = "MDMA_C0IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c0ifcr`] module"]
#[doc(alias = "MDMA_C0IFCR")]
pub type MdmaC0ifcr = crate::Reg<mdma_c0ifcr::MdmaC0ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c0ifcr;
#[doc = "MDMA_C0ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c0esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c0esr`] module"]
#[doc(alias = "MDMA_C0ESR")]
pub type MdmaC0esr = crate::Reg<mdma_c0esr::MdmaC0esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c0esr;
#[doc = "MDMA_C0CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c0cr`] module"]
#[doc(alias = "MDMA_C0CR")]
pub type MdmaC0cr = crate::Reg<mdma_c0cr::MdmaC0crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c0cr;
#[doc = "MDMA_C0TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c0tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c0tcr`] module"]
#[doc(alias = "MDMA_C0TCR")]
pub type MdmaC0tcr = crate::Reg<mdma_c0tcr::MdmaC0tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c0tcr;
#[doc = "MDMA_C0BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c0bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c0bndtr`] module"]
#[doc(alias = "MDMA_C0BNDTR")]
pub type MdmaC0bndtr = crate::Reg<mdma_c0bndtr::MdmaC0bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c0bndtr;
#[doc = "MDMA_C0SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c0sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c0sar`] module"]
#[doc(alias = "MDMA_C0SAR")]
pub type MdmaC0sar = crate::Reg<mdma_c0sar::MdmaC0sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c0sar;
#[doc = "MDMA_C0DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c0dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c0dar`] module"]
#[doc(alias = "MDMA_C0DAR")]
pub type MdmaC0dar = crate::Reg<mdma_c0dar::MdmaC0darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c0dar;
#[doc = "MDMA_C0BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c0brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c0brur`] module"]
#[doc(alias = "MDMA_C0BRUR")]
pub type MdmaC0brur = crate::Reg<mdma_c0brur::MdmaC0brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c0brur;
#[doc = "MDMA_C0LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c0lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c0lar`] module"]
#[doc(alias = "MDMA_C0LAR")]
pub type MdmaC0lar = crate::Reg<mdma_c0lar::MdmaC0larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c0lar;
#[doc = "MDMA_C0TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c0tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c0tbr`] module"]
#[doc(alias = "MDMA_C0TBR")]
pub type MdmaC0tbr = crate::Reg<mdma_c0tbr::MdmaC0tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c0tbr;
#[doc = "MDMA_C0MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c0mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c0mar`] module"]
#[doc(alias = "MDMA_C0MAR")]
pub type MdmaC0mar = crate::Reg<mdma_c0mar::MdmaC0marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c0mar;
#[doc = "MDMA_C0MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c0mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c0mdr`] module"]
#[doc(alias = "MDMA_C0MDR")]
pub type MdmaC0mdr = crate::Reg<mdma_c0mdr::MdmaC0mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c0mdr;
#[doc = "MDMA_C1ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c1isr`] module"]
#[doc(alias = "MDMA_C1ISR")]
pub type MdmaC1isr = crate::Reg<mdma_c1isr::MdmaC1isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c1isr;
#[doc = "MDMA_C1IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c1ifcr`] module"]
#[doc(alias = "MDMA_C1IFCR")]
pub type MdmaC1ifcr = crate::Reg<mdma_c1ifcr::MdmaC1ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c1ifcr;
#[doc = "MDMA_C1ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c1esr`] module"]
#[doc(alias = "MDMA_C1ESR")]
pub type MdmaC1esr = crate::Reg<mdma_c1esr::MdmaC1esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c1esr;
#[doc = "MDMA_C1CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c1cr`] module"]
#[doc(alias = "MDMA_C1CR")]
pub type MdmaC1cr = crate::Reg<mdma_c1cr::MdmaC1crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c1cr;
#[doc = "MDMA_C1TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c1tcr`] module"]
#[doc(alias = "MDMA_C1TCR")]
pub type MdmaC1tcr = crate::Reg<mdma_c1tcr::MdmaC1tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c1tcr;
#[doc = "MDMA_C1BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c1bndtr`] module"]
#[doc(alias = "MDMA_C1BNDTR")]
pub type MdmaC1bndtr = crate::Reg<mdma_c1bndtr::MdmaC1bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c1bndtr;
#[doc = "MDMA_C1SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c1sar`] module"]
#[doc(alias = "MDMA_C1SAR")]
pub type MdmaC1sar = crate::Reg<mdma_c1sar::MdmaC1sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c1sar;
#[doc = "MDMA_C1DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c1dar`] module"]
#[doc(alias = "MDMA_C1DAR")]
pub type MdmaC1dar = crate::Reg<mdma_c1dar::MdmaC1darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c1dar;
#[doc = "MDMA_C1BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c1brur`] module"]
#[doc(alias = "MDMA_C1BRUR")]
pub type MdmaC1brur = crate::Reg<mdma_c1brur::MdmaC1brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c1brur;
#[doc = "MDMA_C1LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c1lar`] module"]
#[doc(alias = "MDMA_C1LAR")]
pub type MdmaC1lar = crate::Reg<mdma_c1lar::MdmaC1larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c1lar;
#[doc = "MDMA_C1TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c1tbr`] module"]
#[doc(alias = "MDMA_C1TBR")]
pub type MdmaC1tbr = crate::Reg<mdma_c1tbr::MdmaC1tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c1tbr;
#[doc = "MDMA_C1MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c1mar`] module"]
#[doc(alias = "MDMA_C1MAR")]
pub type MdmaC1mar = crate::Reg<mdma_c1mar::MdmaC1marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c1mar;
#[doc = "MDMA_C1MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c1mdr`] module"]
#[doc(alias = "MDMA_C1MDR")]
pub type MdmaC1mdr = crate::Reg<mdma_c1mdr::MdmaC1mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c1mdr;
#[doc = "MDMA_C2ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c2isr`] module"]
#[doc(alias = "MDMA_C2ISR")]
pub type MdmaC2isr = crate::Reg<mdma_c2isr::MdmaC2isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c2isr;
#[doc = "MDMA_C2IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c2ifcr`] module"]
#[doc(alias = "MDMA_C2IFCR")]
pub type MdmaC2ifcr = crate::Reg<mdma_c2ifcr::MdmaC2ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c2ifcr;
#[doc = "MDMA_C2ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c2esr`] module"]
#[doc(alias = "MDMA_C2ESR")]
pub type MdmaC2esr = crate::Reg<mdma_c2esr::MdmaC2esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c2esr;
#[doc = "MDMA_C2CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c2cr`] module"]
#[doc(alias = "MDMA_C2CR")]
pub type MdmaC2cr = crate::Reg<mdma_c2cr::MdmaC2crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c2cr;
#[doc = "MDMA_C2TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c2tcr`] module"]
#[doc(alias = "MDMA_C2TCR")]
pub type MdmaC2tcr = crate::Reg<mdma_c2tcr::MdmaC2tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c2tcr;
#[doc = "MDMA_C2BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c2bndtr`] module"]
#[doc(alias = "MDMA_C2BNDTR")]
pub type MdmaC2bndtr = crate::Reg<mdma_c2bndtr::MdmaC2bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c2bndtr;
#[doc = "MDMA_C2SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c2sar`] module"]
#[doc(alias = "MDMA_C2SAR")]
pub type MdmaC2sar = crate::Reg<mdma_c2sar::MdmaC2sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c2sar;
#[doc = "MDMA_C2DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c2dar`] module"]
#[doc(alias = "MDMA_C2DAR")]
pub type MdmaC2dar = crate::Reg<mdma_c2dar::MdmaC2darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c2dar;
#[doc = "MDMA_C2BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c2brur`] module"]
#[doc(alias = "MDMA_C2BRUR")]
pub type MdmaC2brur = crate::Reg<mdma_c2brur::MdmaC2brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c2brur;
#[doc = "MDMA_C2LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c2lar`] module"]
#[doc(alias = "MDMA_C2LAR")]
pub type MdmaC2lar = crate::Reg<mdma_c2lar::MdmaC2larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c2lar;
#[doc = "MDMA_C2TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c2tbr`] module"]
#[doc(alias = "MDMA_C2TBR")]
pub type MdmaC2tbr = crate::Reg<mdma_c2tbr::MdmaC2tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c2tbr;
#[doc = "MDMA_C2MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c2mar`] module"]
#[doc(alias = "MDMA_C2MAR")]
pub type MdmaC2mar = crate::Reg<mdma_c2mar::MdmaC2marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c2mar;
#[doc = "MDMA_C2MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c2mdr`] module"]
#[doc(alias = "MDMA_C2MDR")]
pub type MdmaC2mdr = crate::Reg<mdma_c2mdr::MdmaC2mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c2mdr;
#[doc = "MDMA_C3ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c3isr`] module"]
#[doc(alias = "MDMA_C3ISR")]
pub type MdmaC3isr = crate::Reg<mdma_c3isr::MdmaC3isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c3isr;
#[doc = "MDMA_C3IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c3ifcr`] module"]
#[doc(alias = "MDMA_C3IFCR")]
pub type MdmaC3ifcr = crate::Reg<mdma_c3ifcr::MdmaC3ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c3ifcr;
#[doc = "MDMA_C3ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c3esr`] module"]
#[doc(alias = "MDMA_C3ESR")]
pub type MdmaC3esr = crate::Reg<mdma_c3esr::MdmaC3esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c3esr;
#[doc = "MDMA_C3CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c3cr`] module"]
#[doc(alias = "MDMA_C3CR")]
pub type MdmaC3cr = crate::Reg<mdma_c3cr::MdmaC3crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c3cr;
#[doc = "MDMA_C3TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c3tcr`] module"]
#[doc(alias = "MDMA_C3TCR")]
pub type MdmaC3tcr = crate::Reg<mdma_c3tcr::MdmaC3tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c3tcr;
#[doc = "MDMA_C3BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c3bndtr`] module"]
#[doc(alias = "MDMA_C3BNDTR")]
pub type MdmaC3bndtr = crate::Reg<mdma_c3bndtr::MdmaC3bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c3bndtr;
#[doc = "MDMA_C3SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c3sar`] module"]
#[doc(alias = "MDMA_C3SAR")]
pub type MdmaC3sar = crate::Reg<mdma_c3sar::MdmaC3sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c3sar;
#[doc = "MDMA_C3DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c3dar`] module"]
#[doc(alias = "MDMA_C3DAR")]
pub type MdmaC3dar = crate::Reg<mdma_c3dar::MdmaC3darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c3dar;
#[doc = "MDMA_C3BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c3brur`] module"]
#[doc(alias = "MDMA_C3BRUR")]
pub type MdmaC3brur = crate::Reg<mdma_c3brur::MdmaC3brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c3brur;
#[doc = "MDMA_C3LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c3lar`] module"]
#[doc(alias = "MDMA_C3LAR")]
pub type MdmaC3lar = crate::Reg<mdma_c3lar::MdmaC3larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c3lar;
#[doc = "MDMA_C3TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c3tbr`] module"]
#[doc(alias = "MDMA_C3TBR")]
pub type MdmaC3tbr = crate::Reg<mdma_c3tbr::MdmaC3tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c3tbr;
#[doc = "MDMA_C3MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c3mar`] module"]
#[doc(alias = "MDMA_C3MAR")]
pub type MdmaC3mar = crate::Reg<mdma_c3mar::MdmaC3marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c3mar;
#[doc = "MDMA_C3MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c3mdr`] module"]
#[doc(alias = "MDMA_C3MDR")]
pub type MdmaC3mdr = crate::Reg<mdma_c3mdr::MdmaC3mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c3mdr;
#[doc = "MDMA_C4ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c4isr`] module"]
#[doc(alias = "MDMA_C4ISR")]
pub type MdmaC4isr = crate::Reg<mdma_c4isr::MdmaC4isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c4isr;
#[doc = "MDMA_C4IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c4ifcr`] module"]
#[doc(alias = "MDMA_C4IFCR")]
pub type MdmaC4ifcr = crate::Reg<mdma_c4ifcr::MdmaC4ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c4ifcr;
#[doc = "MDMA_C4ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c4esr`] module"]
#[doc(alias = "MDMA_C4ESR")]
pub type MdmaC4esr = crate::Reg<mdma_c4esr::MdmaC4esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c4esr;
#[doc = "MDMA_C4CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c4cr`] module"]
#[doc(alias = "MDMA_C4CR")]
pub type MdmaC4cr = crate::Reg<mdma_c4cr::MdmaC4crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c4cr;
#[doc = "MDMA_C4TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c4tcr`] module"]
#[doc(alias = "MDMA_C4TCR")]
pub type MdmaC4tcr = crate::Reg<mdma_c4tcr::MdmaC4tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c4tcr;
#[doc = "MDMA_C4BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c4bndtr`] module"]
#[doc(alias = "MDMA_C4BNDTR")]
pub type MdmaC4bndtr = crate::Reg<mdma_c4bndtr::MdmaC4bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c4bndtr;
#[doc = "MDMA_C4SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c4sar`] module"]
#[doc(alias = "MDMA_C4SAR")]
pub type MdmaC4sar = crate::Reg<mdma_c4sar::MdmaC4sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c4sar;
#[doc = "MDMA_C4DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c4dar`] module"]
#[doc(alias = "MDMA_C4DAR")]
pub type MdmaC4dar = crate::Reg<mdma_c4dar::MdmaC4darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c4dar;
#[doc = "MDMA_C4BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c4brur`] module"]
#[doc(alias = "MDMA_C4BRUR")]
pub type MdmaC4brur = crate::Reg<mdma_c4brur::MdmaC4brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c4brur;
#[doc = "MDMA_C4LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c4lar`] module"]
#[doc(alias = "MDMA_C4LAR")]
pub type MdmaC4lar = crate::Reg<mdma_c4lar::MdmaC4larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c4lar;
#[doc = "MDMA_C4TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c4tbr`] module"]
#[doc(alias = "MDMA_C4TBR")]
pub type MdmaC4tbr = crate::Reg<mdma_c4tbr::MdmaC4tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c4tbr;
#[doc = "MDMA_C4MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c4mar`] module"]
#[doc(alias = "MDMA_C4MAR")]
pub type MdmaC4mar = crate::Reg<mdma_c4mar::MdmaC4marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c4mar;
#[doc = "MDMA_C4MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c4mdr`] module"]
#[doc(alias = "MDMA_C4MDR")]
pub type MdmaC4mdr = crate::Reg<mdma_c4mdr::MdmaC4mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c4mdr;
#[doc = "MDMA_C5ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c5isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c5isr`] module"]
#[doc(alias = "MDMA_C5ISR")]
pub type MdmaC5isr = crate::Reg<mdma_c5isr::MdmaC5isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c5isr;
#[doc = "MDMA_C5IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c5ifcr`] module"]
#[doc(alias = "MDMA_C5IFCR")]
pub type MdmaC5ifcr = crate::Reg<mdma_c5ifcr::MdmaC5ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c5ifcr;
#[doc = "MDMA_C5ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c5esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c5esr`] module"]
#[doc(alias = "MDMA_C5ESR")]
pub type MdmaC5esr = crate::Reg<mdma_c5esr::MdmaC5esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c5esr;
#[doc = "MDMA_C5CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c5cr`] module"]
#[doc(alias = "MDMA_C5CR")]
pub type MdmaC5cr = crate::Reg<mdma_c5cr::MdmaC5crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c5cr;
#[doc = "MDMA_C5TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c5tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c5tcr`] module"]
#[doc(alias = "MDMA_C5TCR")]
pub type MdmaC5tcr = crate::Reg<mdma_c5tcr::MdmaC5tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c5tcr;
#[doc = "MDMA_C5BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c5bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c5bndtr`] module"]
#[doc(alias = "MDMA_C5BNDTR")]
pub type MdmaC5bndtr = crate::Reg<mdma_c5bndtr::MdmaC5bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c5bndtr;
#[doc = "MDMA_C5SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c5sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c5sar`] module"]
#[doc(alias = "MDMA_C5SAR")]
pub type MdmaC5sar = crate::Reg<mdma_c5sar::MdmaC5sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c5sar;
#[doc = "MDMA_C5DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c5dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c5dar`] module"]
#[doc(alias = "MDMA_C5DAR")]
pub type MdmaC5dar = crate::Reg<mdma_c5dar::MdmaC5darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c5dar;
#[doc = "MDMA_C5BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c5brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c5brur`] module"]
#[doc(alias = "MDMA_C5BRUR")]
pub type MdmaC5brur = crate::Reg<mdma_c5brur::MdmaC5brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c5brur;
#[doc = "MDMA_C5LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c5lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c5lar`] module"]
#[doc(alias = "MDMA_C5LAR")]
pub type MdmaC5lar = crate::Reg<mdma_c5lar::MdmaC5larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c5lar;
#[doc = "MDMA_C5TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c5tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c5tbr`] module"]
#[doc(alias = "MDMA_C5TBR")]
pub type MdmaC5tbr = crate::Reg<mdma_c5tbr::MdmaC5tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c5tbr;
#[doc = "MDMA_C5MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c5mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c5mar`] module"]
#[doc(alias = "MDMA_C5MAR")]
pub type MdmaC5mar = crate::Reg<mdma_c5mar::MdmaC5marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c5mar;
#[doc = "MDMA_C5MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c5mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c5mdr`] module"]
#[doc(alias = "MDMA_C5MDR")]
pub type MdmaC5mdr = crate::Reg<mdma_c5mdr::MdmaC5mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c5mdr;
#[doc = "MDMA_C6ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c6isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c6isr`] module"]
#[doc(alias = "MDMA_C6ISR")]
pub type MdmaC6isr = crate::Reg<mdma_c6isr::MdmaC6isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c6isr;
#[doc = "MDMA_C6IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c6ifcr`] module"]
#[doc(alias = "MDMA_C6IFCR")]
pub type MdmaC6ifcr = crate::Reg<mdma_c6ifcr::MdmaC6ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c6ifcr;
#[doc = "MDMA_C6ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c6esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c6esr`] module"]
#[doc(alias = "MDMA_C6ESR")]
pub type MdmaC6esr = crate::Reg<mdma_c6esr::MdmaC6esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c6esr;
#[doc = "MDMA_C6CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c6cr`] module"]
#[doc(alias = "MDMA_C6CR")]
pub type MdmaC6cr = crate::Reg<mdma_c6cr::MdmaC6crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c6cr;
#[doc = "MDMA_C6TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c6tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c6tcr`] module"]
#[doc(alias = "MDMA_C6TCR")]
pub type MdmaC6tcr = crate::Reg<mdma_c6tcr::MdmaC6tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c6tcr;
#[doc = "MDMA_C6BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c6bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c6bndtr`] module"]
#[doc(alias = "MDMA_C6BNDTR")]
pub type MdmaC6bndtr = crate::Reg<mdma_c6bndtr::MdmaC6bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c6bndtr;
#[doc = "MDMA_C6SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c6sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c6sar`] module"]
#[doc(alias = "MDMA_C6SAR")]
pub type MdmaC6sar = crate::Reg<mdma_c6sar::MdmaC6sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c6sar;
#[doc = "MDMA_C6DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c6dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c6dar`] module"]
#[doc(alias = "MDMA_C6DAR")]
pub type MdmaC6dar = crate::Reg<mdma_c6dar::MdmaC6darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c6dar;
#[doc = "MDMA_C6BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c6brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c6brur`] module"]
#[doc(alias = "MDMA_C6BRUR")]
pub type MdmaC6brur = crate::Reg<mdma_c6brur::MdmaC6brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c6brur;
#[doc = "MDMA_C6LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c6lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c6lar`] module"]
#[doc(alias = "MDMA_C6LAR")]
pub type MdmaC6lar = crate::Reg<mdma_c6lar::MdmaC6larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c6lar;
#[doc = "MDMA_C6TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c6tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c6tbr`] module"]
#[doc(alias = "MDMA_C6TBR")]
pub type MdmaC6tbr = crate::Reg<mdma_c6tbr::MdmaC6tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c6tbr;
#[doc = "MDMA_C6MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c6mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c6mar`] module"]
#[doc(alias = "MDMA_C6MAR")]
pub type MdmaC6mar = crate::Reg<mdma_c6mar::MdmaC6marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c6mar;
#[doc = "MDMA_C6MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c6mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c6mdr`] module"]
#[doc(alias = "MDMA_C6MDR")]
pub type MdmaC6mdr = crate::Reg<mdma_c6mdr::MdmaC6mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c6mdr;
#[doc = "MDMA_C7ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c7isr`] module"]
#[doc(alias = "MDMA_C7ISR")]
pub type MdmaC7isr = crate::Reg<mdma_c7isr::MdmaC7isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c7isr;
#[doc = "MDMA_C7IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c7ifcr`] module"]
#[doc(alias = "MDMA_C7IFCR")]
pub type MdmaC7ifcr = crate::Reg<mdma_c7ifcr::MdmaC7ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c7ifcr;
#[doc = "MDMA_C7ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c7esr`] module"]
#[doc(alias = "MDMA_C7ESR")]
pub type MdmaC7esr = crate::Reg<mdma_c7esr::MdmaC7esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c7esr;
#[doc = "MDMA_C7CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c7cr`] module"]
#[doc(alias = "MDMA_C7CR")]
pub type MdmaC7cr = crate::Reg<mdma_c7cr::MdmaC7crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c7cr;
#[doc = "MDMA_C7TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c7tcr`] module"]
#[doc(alias = "MDMA_C7TCR")]
pub type MdmaC7tcr = crate::Reg<mdma_c7tcr::MdmaC7tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c7tcr;
#[doc = "MDMA_C7BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c7bndtr`] module"]
#[doc(alias = "MDMA_C7BNDTR")]
pub type MdmaC7bndtr = crate::Reg<mdma_c7bndtr::MdmaC7bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c7bndtr;
#[doc = "MDMA_C7SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c7sar`] module"]
#[doc(alias = "MDMA_C7SAR")]
pub type MdmaC7sar = crate::Reg<mdma_c7sar::MdmaC7sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c7sar;
#[doc = "MDMA_C7DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c7dar`] module"]
#[doc(alias = "MDMA_C7DAR")]
pub type MdmaC7dar = crate::Reg<mdma_c7dar::MdmaC7darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c7dar;
#[doc = "MDMA_C7BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c7brur`] module"]
#[doc(alias = "MDMA_C7BRUR")]
pub type MdmaC7brur = crate::Reg<mdma_c7brur::MdmaC7brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c7brur;
#[doc = "MDMA_C7LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c7lar`] module"]
#[doc(alias = "MDMA_C7LAR")]
pub type MdmaC7lar = crate::Reg<mdma_c7lar::MdmaC7larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c7lar;
#[doc = "MDMA_C7TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c7tbr`] module"]
#[doc(alias = "MDMA_C7TBR")]
pub type MdmaC7tbr = crate::Reg<mdma_c7tbr::MdmaC7tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c7tbr;
#[doc = "MDMA_C7MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c7mar`] module"]
#[doc(alias = "MDMA_C7MAR")]
pub type MdmaC7mar = crate::Reg<mdma_c7mar::MdmaC7marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c7mar;
#[doc = "MDMA_C7MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c7mdr`] module"]
#[doc(alias = "MDMA_C7MDR")]
pub type MdmaC7mdr = crate::Reg<mdma_c7mdr::MdmaC7mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c7mdr;
#[doc = "MDMA_C8ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c8isr`] module"]
#[doc(alias = "MDMA_C8ISR")]
pub type MdmaC8isr = crate::Reg<mdma_c8isr::MdmaC8isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c8isr;
#[doc = "MDMA_C8IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c8ifcr`] module"]
#[doc(alias = "MDMA_C8IFCR")]
pub type MdmaC8ifcr = crate::Reg<mdma_c8ifcr::MdmaC8ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c8ifcr;
#[doc = "MDMA_C8ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c8esr`] module"]
#[doc(alias = "MDMA_C8ESR")]
pub type MdmaC8esr = crate::Reg<mdma_c8esr::MdmaC8esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c8esr;
#[doc = "MDMA_C8CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c8cr`] module"]
#[doc(alias = "MDMA_C8CR")]
pub type MdmaC8cr = crate::Reg<mdma_c8cr::MdmaC8crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c8cr;
#[doc = "MDMA_C8TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c8tcr`] module"]
#[doc(alias = "MDMA_C8TCR")]
pub type MdmaC8tcr = crate::Reg<mdma_c8tcr::MdmaC8tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c8tcr;
#[doc = "MDMA_C8BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c8bndtr`] module"]
#[doc(alias = "MDMA_C8BNDTR")]
pub type MdmaC8bndtr = crate::Reg<mdma_c8bndtr::MdmaC8bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c8bndtr;
#[doc = "MDMA_C8SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c8sar`] module"]
#[doc(alias = "MDMA_C8SAR")]
pub type MdmaC8sar = crate::Reg<mdma_c8sar::MdmaC8sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c8sar;
#[doc = "MDMA_C8DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c8dar`] module"]
#[doc(alias = "MDMA_C8DAR")]
pub type MdmaC8dar = crate::Reg<mdma_c8dar::MdmaC8darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c8dar;
#[doc = "MDMA_C8BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c8brur`] module"]
#[doc(alias = "MDMA_C8BRUR")]
pub type MdmaC8brur = crate::Reg<mdma_c8brur::MdmaC8brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c8brur;
#[doc = "MDMA_C8LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c8lar`] module"]
#[doc(alias = "MDMA_C8LAR")]
pub type MdmaC8lar = crate::Reg<mdma_c8lar::MdmaC8larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c8lar;
#[doc = "MDMA_C8TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c8tbr`] module"]
#[doc(alias = "MDMA_C8TBR")]
pub type MdmaC8tbr = crate::Reg<mdma_c8tbr::MdmaC8tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c8tbr;
#[doc = "MDMA_C8MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c8mar`] module"]
#[doc(alias = "MDMA_C8MAR")]
pub type MdmaC8mar = crate::Reg<mdma_c8mar::MdmaC8marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c8mar;
#[doc = "MDMA_C8MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c8mdr`] module"]
#[doc(alias = "MDMA_C8MDR")]
pub type MdmaC8mdr = crate::Reg<mdma_c8mdr::MdmaC8mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c8mdr;
#[doc = "MDMA_C9ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c9isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c9isr`] module"]
#[doc(alias = "MDMA_C9ISR")]
pub type MdmaC9isr = crate::Reg<mdma_c9isr::MdmaC9isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c9isr;
#[doc = "MDMA_C9IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c9ifcr`] module"]
#[doc(alias = "MDMA_C9IFCR")]
pub type MdmaC9ifcr = crate::Reg<mdma_c9ifcr::MdmaC9ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c9ifcr;
#[doc = "MDMA_C9ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c9esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c9esr`] module"]
#[doc(alias = "MDMA_C9ESR")]
pub type MdmaC9esr = crate::Reg<mdma_c9esr::MdmaC9esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c9esr;
#[doc = "MDMA_C9CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c9cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c9cr`] module"]
#[doc(alias = "MDMA_C9CR")]
pub type MdmaC9cr = crate::Reg<mdma_c9cr::MdmaC9crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c9cr;
#[doc = "MDMA_C9TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c9tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c9tcr`] module"]
#[doc(alias = "MDMA_C9TCR")]
pub type MdmaC9tcr = crate::Reg<mdma_c9tcr::MdmaC9tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c9tcr;
#[doc = "MDMA_C9BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c9bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c9bndtr`] module"]
#[doc(alias = "MDMA_C9BNDTR")]
pub type MdmaC9bndtr = crate::Reg<mdma_c9bndtr::MdmaC9bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c9bndtr;
#[doc = "MDMA_C9SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c9sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c9sar`] module"]
#[doc(alias = "MDMA_C9SAR")]
pub type MdmaC9sar = crate::Reg<mdma_c9sar::MdmaC9sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c9sar;
#[doc = "MDMA_C9DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c9dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c9dar`] module"]
#[doc(alias = "MDMA_C9DAR")]
pub type MdmaC9dar = crate::Reg<mdma_c9dar::MdmaC9darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c9dar;
#[doc = "MDMA_C9BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c9brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c9brur`] module"]
#[doc(alias = "MDMA_C9BRUR")]
pub type MdmaC9brur = crate::Reg<mdma_c9brur::MdmaC9brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c9brur;
#[doc = "MDMA_C9LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c9lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c9lar`] module"]
#[doc(alias = "MDMA_C9LAR")]
pub type MdmaC9lar = crate::Reg<mdma_c9lar::MdmaC9larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c9lar;
#[doc = "MDMA_C9TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c9tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c9tbr`] module"]
#[doc(alias = "MDMA_C9TBR")]
pub type MdmaC9tbr = crate::Reg<mdma_c9tbr::MdmaC9tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c9tbr;
#[doc = "MDMA_C9MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c9mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c9mar`] module"]
#[doc(alias = "MDMA_C9MAR")]
pub type MdmaC9mar = crate::Reg<mdma_c9mar::MdmaC9marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c9mar;
#[doc = "MDMA_C9MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c9mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c9mdr`] module"]
#[doc(alias = "MDMA_C9MDR")]
pub type MdmaC9mdr = crate::Reg<mdma_c9mdr::MdmaC9mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c9mdr;
#[doc = "MDMA_C10ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c10isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c10isr`] module"]
#[doc(alias = "MDMA_C10ISR")]
pub type MdmaC10isr = crate::Reg<mdma_c10isr::MdmaC10isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c10isr;
#[doc = "MDMA_C10IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c10ifcr`] module"]
#[doc(alias = "MDMA_C10IFCR")]
pub type MdmaC10ifcr = crate::Reg<mdma_c10ifcr::MdmaC10ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c10ifcr;
#[doc = "MDMA_C10ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c10esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c10esr`] module"]
#[doc(alias = "MDMA_C10ESR")]
pub type MdmaC10esr = crate::Reg<mdma_c10esr::MdmaC10esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c10esr;
#[doc = "MDMA_C10CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c10cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c10cr`] module"]
#[doc(alias = "MDMA_C10CR")]
pub type MdmaC10cr = crate::Reg<mdma_c10cr::MdmaC10crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c10cr;
#[doc = "MDMA_C10TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c10tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c10tcr`] module"]
#[doc(alias = "MDMA_C10TCR")]
pub type MdmaC10tcr = crate::Reg<mdma_c10tcr::MdmaC10tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c10tcr;
#[doc = "MDMA_C10BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c10bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c10bndtr`] module"]
#[doc(alias = "MDMA_C10BNDTR")]
pub type MdmaC10bndtr = crate::Reg<mdma_c10bndtr::MdmaC10bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c10bndtr;
#[doc = "MDMA_C10SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c10sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c10sar`] module"]
#[doc(alias = "MDMA_C10SAR")]
pub type MdmaC10sar = crate::Reg<mdma_c10sar::MdmaC10sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c10sar;
#[doc = "MDMA_C10DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c10dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c10dar`] module"]
#[doc(alias = "MDMA_C10DAR")]
pub type MdmaC10dar = crate::Reg<mdma_c10dar::MdmaC10darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c10dar;
#[doc = "MDMA_C10BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c10brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c10brur`] module"]
#[doc(alias = "MDMA_C10BRUR")]
pub type MdmaC10brur = crate::Reg<mdma_c10brur::MdmaC10brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c10brur;
#[doc = "MDMA_C10LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c10lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c10lar`] module"]
#[doc(alias = "MDMA_C10LAR")]
pub type MdmaC10lar = crate::Reg<mdma_c10lar::MdmaC10larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c10lar;
#[doc = "MDMA_C10TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c10tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c10tbr`] module"]
#[doc(alias = "MDMA_C10TBR")]
pub type MdmaC10tbr = crate::Reg<mdma_c10tbr::MdmaC10tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c10tbr;
#[doc = "MDMA_C10MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c10mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c10mar`] module"]
#[doc(alias = "MDMA_C10MAR")]
pub type MdmaC10mar = crate::Reg<mdma_c10mar::MdmaC10marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c10mar;
#[doc = "MDMA_C10MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c10mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c10mdr`] module"]
#[doc(alias = "MDMA_C10MDR")]
pub type MdmaC10mdr = crate::Reg<mdma_c10mdr::MdmaC10mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c10mdr;
#[doc = "MDMA_C11ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c11isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c11isr`] module"]
#[doc(alias = "MDMA_C11ISR")]
pub type MdmaC11isr = crate::Reg<mdma_c11isr::MdmaC11isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c11isr;
#[doc = "MDMA_C11IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c11ifcr`] module"]
#[doc(alias = "MDMA_C11IFCR")]
pub type MdmaC11ifcr = crate::Reg<mdma_c11ifcr::MdmaC11ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c11ifcr;
#[doc = "MDMA_C11ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c11esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c11esr`] module"]
#[doc(alias = "MDMA_C11ESR")]
pub type MdmaC11esr = crate::Reg<mdma_c11esr::MdmaC11esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c11esr;
#[doc = "MDMA_C11CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c11cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c11cr`] module"]
#[doc(alias = "MDMA_C11CR")]
pub type MdmaC11cr = crate::Reg<mdma_c11cr::MdmaC11crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c11cr;
#[doc = "MDMA_C11TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c11tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c11tcr`] module"]
#[doc(alias = "MDMA_C11TCR")]
pub type MdmaC11tcr = crate::Reg<mdma_c11tcr::MdmaC11tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c11tcr;
#[doc = "MDMA_C11BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c11bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c11bndtr`] module"]
#[doc(alias = "MDMA_C11BNDTR")]
pub type MdmaC11bndtr = crate::Reg<mdma_c11bndtr::MdmaC11bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c11bndtr;
#[doc = "MDMA_C11SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c11sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c11sar`] module"]
#[doc(alias = "MDMA_C11SAR")]
pub type MdmaC11sar = crate::Reg<mdma_c11sar::MdmaC11sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c11sar;
#[doc = "MDMA_C11DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c11dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c11dar`] module"]
#[doc(alias = "MDMA_C11DAR")]
pub type MdmaC11dar = crate::Reg<mdma_c11dar::MdmaC11darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c11dar;
#[doc = "MDMA_C11BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c11brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c11brur`] module"]
#[doc(alias = "MDMA_C11BRUR")]
pub type MdmaC11brur = crate::Reg<mdma_c11brur::MdmaC11brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c11brur;
#[doc = "MDMA_C11LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c11lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c11lar`] module"]
#[doc(alias = "MDMA_C11LAR")]
pub type MdmaC11lar = crate::Reg<mdma_c11lar::MdmaC11larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c11lar;
#[doc = "MDMA_C11TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c11tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c11tbr`] module"]
#[doc(alias = "MDMA_C11TBR")]
pub type MdmaC11tbr = crate::Reg<mdma_c11tbr::MdmaC11tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c11tbr;
#[doc = "MDMA_C11MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c11mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c11mar`] module"]
#[doc(alias = "MDMA_C11MAR")]
pub type MdmaC11mar = crate::Reg<mdma_c11mar::MdmaC11marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c11mar;
#[doc = "MDMA_C11MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c11mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c11mdr`] module"]
#[doc(alias = "MDMA_C11MDR")]
pub type MdmaC11mdr = crate::Reg<mdma_c11mdr::MdmaC11mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c11mdr;
#[doc = "MDMA_C12ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c12isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c12isr`] module"]
#[doc(alias = "MDMA_C12ISR")]
pub type MdmaC12isr = crate::Reg<mdma_c12isr::MdmaC12isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c12isr;
#[doc = "MDMA_C12IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c12ifcr`] module"]
#[doc(alias = "MDMA_C12IFCR")]
pub type MdmaC12ifcr = crate::Reg<mdma_c12ifcr::MdmaC12ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c12ifcr;
#[doc = "MDMA_C12ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c12esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c12esr`] module"]
#[doc(alias = "MDMA_C12ESR")]
pub type MdmaC12esr = crate::Reg<mdma_c12esr::MdmaC12esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c12esr;
#[doc = "MDMA_C12CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c12cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c12cr`] module"]
#[doc(alias = "MDMA_C12CR")]
pub type MdmaC12cr = crate::Reg<mdma_c12cr::MdmaC12crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c12cr;
#[doc = "MDMA_C12TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c12tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c12tcr`] module"]
#[doc(alias = "MDMA_C12TCR")]
pub type MdmaC12tcr = crate::Reg<mdma_c12tcr::MdmaC12tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c12tcr;
#[doc = "MDMA_C12BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c12bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c12bndtr`] module"]
#[doc(alias = "MDMA_C12BNDTR")]
pub type MdmaC12bndtr = crate::Reg<mdma_c12bndtr::MdmaC12bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c12bndtr;
#[doc = "MDMA_C12SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c12sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c12sar`] module"]
#[doc(alias = "MDMA_C12SAR")]
pub type MdmaC12sar = crate::Reg<mdma_c12sar::MdmaC12sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c12sar;
#[doc = "MDMA_C12DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c12dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c12dar`] module"]
#[doc(alias = "MDMA_C12DAR")]
pub type MdmaC12dar = crate::Reg<mdma_c12dar::MdmaC12darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c12dar;
#[doc = "MDMA_C12BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c12brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c12brur`] module"]
#[doc(alias = "MDMA_C12BRUR")]
pub type MdmaC12brur = crate::Reg<mdma_c12brur::MdmaC12brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c12brur;
#[doc = "MDMA_C12LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c12lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c12lar`] module"]
#[doc(alias = "MDMA_C12LAR")]
pub type MdmaC12lar = crate::Reg<mdma_c12lar::MdmaC12larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c12lar;
#[doc = "MDMA_C12TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c12tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c12tbr`] module"]
#[doc(alias = "MDMA_C12TBR")]
pub type MdmaC12tbr = crate::Reg<mdma_c12tbr::MdmaC12tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c12tbr;
#[doc = "MDMA_C12MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c12mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c12mar`] module"]
#[doc(alias = "MDMA_C12MAR")]
pub type MdmaC12mar = crate::Reg<mdma_c12mar::MdmaC12marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c12mar;
#[doc = "MDMA_C12MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c12mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c12mdr`] module"]
#[doc(alias = "MDMA_C12MDR")]
pub type MdmaC12mdr = crate::Reg<mdma_c12mdr::MdmaC12mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c12mdr;
#[doc = "MDMA_C13ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c13isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c13isr`] module"]
#[doc(alias = "MDMA_C13ISR")]
pub type MdmaC13isr = crate::Reg<mdma_c13isr::MdmaC13isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c13isr;
#[doc = "MDMA_C13IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c13ifcr`] module"]
#[doc(alias = "MDMA_C13IFCR")]
pub type MdmaC13ifcr = crate::Reg<mdma_c13ifcr::MdmaC13ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c13ifcr;
#[doc = "MDMA_C13ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c13esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c13esr`] module"]
#[doc(alias = "MDMA_C13ESR")]
pub type MdmaC13esr = crate::Reg<mdma_c13esr::MdmaC13esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c13esr;
#[doc = "MDMA_C13CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c13cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c13cr`] module"]
#[doc(alias = "MDMA_C13CR")]
pub type MdmaC13cr = crate::Reg<mdma_c13cr::MdmaC13crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c13cr;
#[doc = "MDMA_C13TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c13tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c13tcr`] module"]
#[doc(alias = "MDMA_C13TCR")]
pub type MdmaC13tcr = crate::Reg<mdma_c13tcr::MdmaC13tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c13tcr;
#[doc = "MDMA_C13BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c13bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c13bndtr`] module"]
#[doc(alias = "MDMA_C13BNDTR")]
pub type MdmaC13bndtr = crate::Reg<mdma_c13bndtr::MdmaC13bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c13bndtr;
#[doc = "MDMA_C13SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c13sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c13sar`] module"]
#[doc(alias = "MDMA_C13SAR")]
pub type MdmaC13sar = crate::Reg<mdma_c13sar::MdmaC13sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c13sar;
#[doc = "MDMA_C13DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c13dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c13dar`] module"]
#[doc(alias = "MDMA_C13DAR")]
pub type MdmaC13dar = crate::Reg<mdma_c13dar::MdmaC13darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c13dar;
#[doc = "MDMA_C13BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c13brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c13brur`] module"]
#[doc(alias = "MDMA_C13BRUR")]
pub type MdmaC13brur = crate::Reg<mdma_c13brur::MdmaC13brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c13brur;
#[doc = "MDMA_C13LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c13lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c13lar`] module"]
#[doc(alias = "MDMA_C13LAR")]
pub type MdmaC13lar = crate::Reg<mdma_c13lar::MdmaC13larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c13lar;
#[doc = "MDMA_C13TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c13tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c13tbr`] module"]
#[doc(alias = "MDMA_C13TBR")]
pub type MdmaC13tbr = crate::Reg<mdma_c13tbr::MdmaC13tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c13tbr;
#[doc = "MDMA_C13MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c13mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c13mar`] module"]
#[doc(alias = "MDMA_C13MAR")]
pub type MdmaC13mar = crate::Reg<mdma_c13mar::MdmaC13marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c13mar;
#[doc = "MDMA_C13MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c13mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c13mdr`] module"]
#[doc(alias = "MDMA_C13MDR")]
pub type MdmaC13mdr = crate::Reg<mdma_c13mdr::MdmaC13mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c13mdr;
#[doc = "MDMA_C14ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c14isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c14isr`] module"]
#[doc(alias = "MDMA_C14ISR")]
pub type MdmaC14isr = crate::Reg<mdma_c14isr::MdmaC14isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c14isr;
#[doc = "MDMA_C14IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c14ifcr`] module"]
#[doc(alias = "MDMA_C14IFCR")]
pub type MdmaC14ifcr = crate::Reg<mdma_c14ifcr::MdmaC14ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c14ifcr;
#[doc = "MDMA_C14ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c14esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c14esr`] module"]
#[doc(alias = "MDMA_C14ESR")]
pub type MdmaC14esr = crate::Reg<mdma_c14esr::MdmaC14esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c14esr;
#[doc = "MDMA_C14CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c14cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c14cr`] module"]
#[doc(alias = "MDMA_C14CR")]
pub type MdmaC14cr = crate::Reg<mdma_c14cr::MdmaC14crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c14cr;
#[doc = "MDMA_C14TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c14tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c14tcr`] module"]
#[doc(alias = "MDMA_C14TCR")]
pub type MdmaC14tcr = crate::Reg<mdma_c14tcr::MdmaC14tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c14tcr;
#[doc = "MDMA_C14BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c14bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c14bndtr`] module"]
#[doc(alias = "MDMA_C14BNDTR")]
pub type MdmaC14bndtr = crate::Reg<mdma_c14bndtr::MdmaC14bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c14bndtr;
#[doc = "MDMA_C14SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c14sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c14sar`] module"]
#[doc(alias = "MDMA_C14SAR")]
pub type MdmaC14sar = crate::Reg<mdma_c14sar::MdmaC14sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c14sar;
#[doc = "MDMA_C14DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c14dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c14dar`] module"]
#[doc(alias = "MDMA_C14DAR")]
pub type MdmaC14dar = crate::Reg<mdma_c14dar::MdmaC14darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c14dar;
#[doc = "MDMA_C14BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c14brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c14brur`] module"]
#[doc(alias = "MDMA_C14BRUR")]
pub type MdmaC14brur = crate::Reg<mdma_c14brur::MdmaC14brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c14brur;
#[doc = "MDMA_C14LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c14lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c14lar`] module"]
#[doc(alias = "MDMA_C14LAR")]
pub type MdmaC14lar = crate::Reg<mdma_c14lar::MdmaC14larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c14lar;
#[doc = "MDMA_C14TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c14tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c14tbr`] module"]
#[doc(alias = "MDMA_C14TBR")]
pub type MdmaC14tbr = crate::Reg<mdma_c14tbr::MdmaC14tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c14tbr;
#[doc = "MDMA_C14MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c14mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c14mar`] module"]
#[doc(alias = "MDMA_C14MAR")]
pub type MdmaC14mar = crate::Reg<mdma_c14mar::MdmaC14marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c14mar;
#[doc = "MDMA_C14MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c14mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c14mdr`] module"]
#[doc(alias = "MDMA_C14MDR")]
pub type MdmaC14mdr = crate::Reg<mdma_c14mdr::MdmaC14mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c14mdr;
#[doc = "MDMA_C15ISR (r) register accessor: MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c15isr`] module"]
#[doc(alias = "MDMA_C15ISR")]
pub type MdmaC15isr = crate::Reg<mdma_c15isr::MdmaC15isrSpec>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod mdma_c15isr;
#[doc = "MDMA_C15IFCR (w) register accessor: MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c15ifcr`] module"]
#[doc(alias = "MDMA_C15IFCR")]
pub type MdmaC15ifcr = crate::Reg<mdma_c15ifcr::MdmaC15ifcrSpec>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod mdma_c15ifcr;
#[doc = "MDMA_C15ESR (r) register accessor: MDMA Channel x error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c15esr`] module"]
#[doc(alias = "MDMA_C15ESR")]
pub type MdmaC15esr = crate::Reg<mdma_c15esr::MdmaC15esrSpec>;
#[doc = "MDMA Channel x error status register"]
pub mod mdma_c15esr;
#[doc = "MDMA_C15CR (rw) register accessor: This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c15cr`] module"]
#[doc(alias = "MDMA_C15CR")]
pub type MdmaC15cr = crate::Reg<mdma_c15cr::MdmaC15crSpec>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c15cr;
#[doc = "MDMA_C15TCR (rw) register accessor: This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c15tcr`] module"]
#[doc(alias = "MDMA_C15TCR")]
pub type MdmaC15tcr = crate::Reg<mdma_c15tcr::MdmaC15tcrSpec>;
#[doc = "This register is used to configure the concerned channel."]
pub mod mdma_c15tcr;
#[doc = "MDMA_C15BNDTR (rw) register accessor: MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c15bndtr`] module"]
#[doc(alias = "MDMA_C15BNDTR")]
pub type MdmaC15bndtr = crate::Reg<mdma_c15bndtr::MdmaC15bndtrSpec>;
#[doc = "MDMA Channel x block number of data register"]
pub mod mdma_c15bndtr;
#[doc = "MDMA_C15SAR (rw) register accessor: MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c15sar`] module"]
#[doc(alias = "MDMA_C15SAR")]
pub type MdmaC15sar = crate::Reg<mdma_c15sar::MdmaC15sarSpec>;
#[doc = "MDMA channel x source address register"]
pub mod mdma_c15sar;
#[doc = "MDMA_C15DAR (rw) register accessor: MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c15dar`] module"]
#[doc(alias = "MDMA_C15DAR")]
pub type MdmaC15dar = crate::Reg<mdma_c15dar::MdmaC15darSpec>;
#[doc = "MDMA channel x destination address register"]
pub mod mdma_c15dar;
#[doc = "MDMA_C15BRUR (rw) register accessor: MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c15brur`] module"]
#[doc(alias = "MDMA_C15BRUR")]
pub type MdmaC15brur = crate::Reg<mdma_c15brur::MdmaC15brurSpec>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod mdma_c15brur;
#[doc = "MDMA_C15LAR (rw) register accessor: MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c15lar`] module"]
#[doc(alias = "MDMA_C15LAR")]
pub type MdmaC15lar = crate::Reg<mdma_c15lar::MdmaC15larSpec>;
#[doc = "MDMA channel x Link Address register"]
pub mod mdma_c15lar;
#[doc = "MDMA_C15TBR (rw) register accessor: MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c15tbr`] module"]
#[doc(alias = "MDMA_C15TBR")]
pub type MdmaC15tbr = crate::Reg<mdma_c15tbr::MdmaC15tbrSpec>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod mdma_c15tbr;
#[doc = "MDMA_C15MAR (rw) register accessor: MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c15mar`] module"]
#[doc(alias = "MDMA_C15MAR")]
pub type MdmaC15mar = crate::Reg<mdma_c15mar::MdmaC15marSpec>;
#[doc = "MDMA channel x Mask address register"]
pub mod mdma_c15mar;
#[doc = "MDMA_C15MDR (rw) register accessor: MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdma_c15mdr`] module"]
#[doc(alias = "MDMA_C15MDR")]
pub type MdmaC15mdr = crate::Reg<mdma_c15mdr::MdmaC15mdrSpec>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdma_c15mdr;
