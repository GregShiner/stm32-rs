#[doc = "Register `CONFRN4` reader"]
pub type R = crate::R<Confrn4Spec>;
#[doc = "Register `CONFRN4` writer"]
pub type W = crate::W<Confrn4Spec>;
#[doc = "Field `HD` reader - Huffman DC Selects the Huffman table for encoding the DC coefficients."]
pub type HdR = crate::BitReader;
#[doc = "Field `HD` writer - Huffman DC Selects the Huffman table for encoding the DC coefficients."]
pub type HdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HA` reader - Huffman AC Selects the Huffman table for encoding the AC coefficients."]
pub type HaR = crate::BitReader;
#[doc = "Field `HA` writer - Huffman AC Selects the Huffman table for encoding the AC coefficients."]
pub type HaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QT` reader - Quantization Table Selects quantization table associated with a color component."]
pub type QtR = crate::FieldReader;
#[doc = "Field `QT` writer - Quantization Table Selects quantization table associated with a color component."]
pub type QtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NB` reader - Number of Block Number of data units minus 1 that belong to a particular color in the MCU."]
pub type NbR = crate::FieldReader;
#[doc = "Field `NB` writer - Number of Block Number of data units minus 1 that belong to a particular color in the MCU."]
pub type NbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VSF` reader - Vertical Sampling Factor Vertical sampling factor for component i."]
pub type VsfR = crate::FieldReader;
#[doc = "Field `VSF` writer - Vertical Sampling Factor Vertical sampling factor for component i."]
pub type VsfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HSF` reader - Horizontal Sampling Factor Horizontal sampling factor for component i."]
pub type HsfR = crate::FieldReader;
#[doc = "Field `HSF` writer - Horizontal Sampling Factor Horizontal sampling factor for component i."]
pub type HsfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Huffman DC Selects the Huffman table for encoding the DC coefficients."]
    #[inline(always)]
    pub fn hd(&self) -> HdR {
        HdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Huffman AC Selects the Huffman table for encoding the AC coefficients."]
    #[inline(always)]
    pub fn ha(&self) -> HaR {
        HaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Quantization Table Selects quantization table associated with a color component."]
    #[inline(always)]
    pub fn qt(&self) -> QtR {
        QtR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Number of Block Number of data units minus 1 that belong to a particular color in the MCU."]
    #[inline(always)]
    pub fn nb(&self) -> NbR {
        NbR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Vertical Sampling Factor Vertical sampling factor for component i."]
    #[inline(always)]
    pub fn vsf(&self) -> VsfR {
        VsfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Horizontal Sampling Factor Horizontal sampling factor for component i."]
    #[inline(always)]
    pub fn hsf(&self) -> HsfR {
        HsfR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Huffman DC Selects the Huffman table for encoding the DC coefficients."]
    #[inline(always)]
    pub fn hd(&mut self) -> HdW<Confrn4Spec> {
        HdW::new(self, 0)
    }
    #[doc = "Bit 1 - Huffman AC Selects the Huffman table for encoding the AC coefficients."]
    #[inline(always)]
    pub fn ha(&mut self) -> HaW<Confrn4Spec> {
        HaW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Quantization Table Selects quantization table associated with a color component."]
    #[inline(always)]
    pub fn qt(&mut self) -> QtW<Confrn4Spec> {
        QtW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Number of Block Number of data units minus 1 that belong to a particular color in the MCU."]
    #[inline(always)]
    pub fn nb(&mut self) -> NbW<Confrn4Spec> {
        NbW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Vertical Sampling Factor Vertical sampling factor for component i."]
    #[inline(always)]
    pub fn vsf(&mut self) -> VsfW<Confrn4Spec> {
        VsfW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Horizontal Sampling Factor Horizontal sampling factor for component i."]
    #[inline(always)]
    pub fn hsf(&mut self) -> HsfW<Confrn4Spec> {
        HsfW::new(self, 12)
    }
}
#[doc = "JPEG codec configuration register 4-7\n\nYou can [`read`](crate::Reg::read) this register and get [`confrn4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confrn4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Confrn4Spec;
impl crate::RegisterSpec for Confrn4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`confrn4::R`](R) reader structure"]
impl crate::Readable for Confrn4Spec {}
#[doc = "`write(|w| ..)` method takes [`confrn4::W`](W) writer structure"]
impl crate::Writable for Confrn4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFRN4 to value 0"]
impl crate::Resettable for Confrn4Spec {}
