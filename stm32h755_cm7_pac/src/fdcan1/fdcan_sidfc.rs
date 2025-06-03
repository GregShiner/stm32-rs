#[doc = "Register `FDCAN_SIDFC` reader"]
pub type R = crate::R<FdcanSidfcSpec>;
#[doc = "Register `FDCAN_SIDFC` writer"]
pub type W = crate::W<FdcanSidfcSpec>;
#[doc = "Field `FLSSA` reader - Filter List Standard Start Address"]
pub type FlssaR = crate::FieldReader<u16>;
#[doc = "Field `FLSSA` writer - Filter List Standard Start Address"]
pub type FlssaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LSS` reader - List Size Standard"]
pub type LssR = crate::FieldReader;
#[doc = "Field `LSS` writer - List Size Standard"]
pub type LssW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 2:15 - Filter List Standard Start Address"]
    #[inline(always)]
    pub fn flssa(&self) -> FlssaR {
        FlssaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - List Size Standard"]
    #[inline(always)]
    pub fn lss(&self) -> LssR {
        LssR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Filter List Standard Start Address"]
    #[inline(always)]
    pub fn flssa(&mut self) -> FlssaW<FdcanSidfcSpec> {
        FlssaW::new(self, 2)
    }
    #[doc = "Bits 16:23 - List Size Standard"]
    #[inline(always)]
    pub fn lss(&mut self) -> LssW<FdcanSidfcSpec> {
        LssW::new(self, 16)
    }
}
#[doc = "FDCAN Standard ID Filter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_sidfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_sidfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanSidfcSpec;
impl crate::RegisterSpec for FdcanSidfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_sidfc::R`](R) reader structure"]
impl crate::Readable for FdcanSidfcSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_sidfc::W`](W) writer structure"]
impl crate::Writable for FdcanSidfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_SIDFC to value 0"]
impl crate::Resettable for FdcanSidfcSpec {}
