#[doc = "Register `CONFR1` reader"]
pub type R = crate::R<Confr1Spec>;
#[doc = "Register `CONFR1` writer"]
pub type W = crate::W<Confr1Spec>;
#[doc = "Field `NF` reader - Number of color components This field defines the number of color components minus 1."]
pub type NfR = crate::FieldReader;
#[doc = "Field `NF` writer - Number of color components This field defines the number of color components minus 1."]
pub type NfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DE` reader - Decoding Enable This bit selects the coding or decoding process"]
pub type DeR = crate::BitReader;
#[doc = "Field `DE` writer - Decoding Enable This bit selects the coding or decoding process"]
pub type DeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLORSPACE` reader - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream."]
pub type ColorspaceR = crate::FieldReader;
#[doc = "Field `COLORSPACE` writer - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream."]
pub type ColorspaceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NS` reader - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment."]
pub type NsR = crate::FieldReader;
#[doc = "Field `NS` writer - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment."]
pub type NsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HDR` reader - Header Processing This bit enable the header processing (generation/parsing)."]
pub type HdrR = crate::BitReader;
#[doc = "Field `HDR` writer - Header Processing This bit enable the header processing (generation/parsing)."]
pub type HdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YSIZE` reader - Y Size This field defines the number of lines in source image."]
pub type YsizeR = crate::FieldReader<u16>;
#[doc = "Field `YSIZE` writer - Y Size This field defines the number of lines in source image."]
pub type YsizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - Number of color components This field defines the number of color components minus 1."]
    #[inline(always)]
    pub fn nf(&self) -> NfR {
        NfR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Decoding Enable This bit selects the coding or decoding process"]
    #[inline(always)]
    pub fn de(&self) -> DeR {
        DeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream."]
    #[inline(always)]
    pub fn colorspace(&self) -> ColorspaceR {
        ColorspaceR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment."]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Header Processing This bit enable the header processing (generation/parsing)."]
    #[inline(always)]
    pub fn hdr(&self) -> HdrR {
        HdrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Y Size This field defines the number of lines in source image."]
    #[inline(always)]
    pub fn ysize(&self) -> YsizeR {
        YsizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of color components This field defines the number of color components minus 1."]
    #[inline(always)]
    pub fn nf(&mut self) -> NfW<Confr1Spec> {
        NfW::new(self, 0)
    }
    #[doc = "Bit 3 - Decoding Enable This bit selects the coding or decoding process"]
    #[inline(always)]
    pub fn de(&mut self) -> DeW<Confr1Spec> {
        DeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream."]
    #[inline(always)]
    pub fn colorspace(&mut self) -> ColorspaceW<Confr1Spec> {
        ColorspaceW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment."]
    #[inline(always)]
    pub fn ns(&mut self) -> NsW<Confr1Spec> {
        NsW::new(self, 6)
    }
    #[doc = "Bit 8 - Header Processing This bit enable the header processing (generation/parsing)."]
    #[inline(always)]
    pub fn hdr(&mut self) -> HdrW<Confr1Spec> {
        HdrW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Y Size This field defines the number of lines in source image."]
    #[inline(always)]
    pub fn ysize(&mut self) -> YsizeW<Confr1Spec> {
        YsizeW::new(self, 16)
    }
}
#[doc = "JPEG codec configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`confr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Confr1Spec;
impl crate::RegisterSpec for Confr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`confr1::R`](R) reader structure"]
impl crate::Readable for Confr1Spec {}
#[doc = "`write(|w| ..)` method takes [`confr1::W`](W) writer structure"]
impl crate::Writable for Confr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFR1 to value 0"]
impl crate::Resettable for Confr1Spec {}
