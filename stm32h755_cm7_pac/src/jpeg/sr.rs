#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `IFTF` reader - Input FIFO Threshold Flag This bit is set when the input FIFO is not full and is bellow its threshold."]
pub type IftfR = crate::BitReader;
#[doc = "Field `IFNFF` reader - Input FIFO Not Full Flag This bit is set when the input FIFO is not full (a data can be written)."]
pub type IfnffR = crate::BitReader;
#[doc = "Field `OFTF` reader - Output FIFO Threshold Flag This bit is set when the output FIFO is not empty and has reach its threshold."]
pub type OftfR = crate::BitReader;
#[doc = "Field `OFNEF` reader - Output FIFO Not Empty Flag This bit is set when the output FIFO is not empty (a data is available)."]
pub type OfnefR = crate::BitReader;
#[doc = "Field `EOCF` reader - End of Conversion Flag This bit is set when the JPEG codec core has finished the encoding or the decoding process and than last data has been sent to the output FIFO."]
pub type EocfR = crate::BitReader;
#[doc = "Field `HPDF` reader - Header Parsing Done Flag This bit is set in decode mode when the JPEG codec has finished the parsing of the headers and the internal registers have been updated."]
pub type HpdfR = crate::BitReader;
#[doc = "Field `COF` reader - Codec Operation Flag This bit is set when when a JPEG codec operation is on going (encoding or decoding)."]
pub type CofR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Input FIFO Threshold Flag This bit is set when the input FIFO is not full and is bellow its threshold."]
    #[inline(always)]
    pub fn iftf(&self) -> IftfR {
        IftfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input FIFO Not Full Flag This bit is set when the input FIFO is not full (a data can be written)."]
    #[inline(always)]
    pub fn ifnff(&self) -> IfnffR {
        IfnffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output FIFO Threshold Flag This bit is set when the output FIFO is not empty and has reach its threshold."]
    #[inline(always)]
    pub fn oftf(&self) -> OftfR {
        OftfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Flag This bit is set when the output FIFO is not empty (a data is available)."]
    #[inline(always)]
    pub fn ofnef(&self) -> OfnefR {
        OfnefR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Flag This bit is set when the JPEG codec core has finished the encoding or the decoding process and than last data has been sent to the output FIFO."]
    #[inline(always)]
    pub fn eocf(&self) -> EocfR {
        EocfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Header Parsing Done Flag This bit is set in decode mode when the JPEG codec has finished the parsing of the headers and the internal registers have been updated."]
    #[inline(always)]
    pub fn hpdf(&self) -> HpdfR {
        HpdfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Codec Operation Flag This bit is set when when a JPEG codec operation is on going (encoding or decoding)."]
    #[inline(always)]
    pub fn cof(&self) -> CofR {
        CofR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "JPEG status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0x06"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x06;
}
