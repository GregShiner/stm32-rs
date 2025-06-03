#[doc = "Register `MACIVIR` reader"]
pub type R = crate::R<MacivirSpec>;
#[doc = "Register `MACIVIR` writer"]
pub type W = crate::W<MacivirSpec>;
#[doc = "Field `VLT` reader - VLT"]
pub type VltR = crate::FieldReader<u16>;
#[doc = "Field `VLT` writer - VLT"]
pub type VltW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VLC` reader - VLC"]
pub type VlcR = crate::FieldReader;
#[doc = "Field `VLC` writer - VLC"]
pub type VlcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VLP` reader - VLP"]
pub type VlpR = crate::BitReader;
#[doc = "Field `VLP` writer - VLP"]
pub type VlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSVL` reader - CSVL"]
pub type CsvlR = crate::BitReader;
#[doc = "Field `CSVL` writer - CSVL"]
pub type CsvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLTI` reader - VLTI"]
pub type VltiR = crate::BitReader;
#[doc = "Field `VLTI` writer - VLTI"]
pub type VltiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLT"]
    #[inline(always)]
    pub fn vlt(&self) -> VltR {
        VltR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLC"]
    #[inline(always)]
    pub fn vlc(&self) -> VlcR {
        VlcR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - VLP"]
    #[inline(always)]
    pub fn vlp(&self) -> VlpR {
        VlpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CSVL"]
    #[inline(always)]
    pub fn csvl(&self) -> CsvlR {
        CsvlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VLTI"]
    #[inline(always)]
    pub fn vlti(&self) -> VltiR {
        VltiR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLT"]
    #[inline(always)]
    pub fn vlt(&mut self) -> VltW<MacivirSpec> {
        VltW::new(self, 0)
    }
    #[doc = "Bits 16:17 - VLC"]
    #[inline(always)]
    pub fn vlc(&mut self) -> VlcW<MacivirSpec> {
        VlcW::new(self, 16)
    }
    #[doc = "Bit 18 - VLP"]
    #[inline(always)]
    pub fn vlp(&mut self) -> VlpW<MacivirSpec> {
        VlpW::new(self, 18)
    }
    #[doc = "Bit 19 - CSVL"]
    #[inline(always)]
    pub fn csvl(&mut self) -> CsvlW<MacivirSpec> {
        CsvlW::new(self, 19)
    }
    #[doc = "Bit 20 - VLTI"]
    #[inline(always)]
    pub fn vlti(&mut self) -> VltiW<MacivirSpec> {
        VltiW::new(self, 20)
    }
}
#[doc = "Inner VLAN inclusion register\n\nYou can [`read`](crate::Reg::read) this register and get [`macivir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macivir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacivirSpec;
impl crate::RegisterSpec for MacivirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macivir::R`](R) reader structure"]
impl crate::Readable for MacivirSpec {}
#[doc = "`write(|w| ..)` method takes [`macivir::W`](W) writer structure"]
impl crate::Writable for MacivirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACIVIR to value 0"]
impl crate::Resettable for MacivirSpec {}
