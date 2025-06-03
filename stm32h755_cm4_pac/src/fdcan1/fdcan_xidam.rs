#[doc = "Register `FDCAN_XIDAM` reader"]
pub type R = crate::R<FdcanXidamSpec>;
#[doc = "Register `FDCAN_XIDAM` writer"]
pub type W = crate::W<FdcanXidamSpec>;
#[doc = "Field `EIDM` reader - Extended ID Mask"]
pub type EidmR = crate::FieldReader<u32>;
#[doc = "Field `EIDM` writer - Extended ID Mask"]
pub type EidmW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - Extended ID Mask"]
    #[inline(always)]
    pub fn eidm(&self) -> EidmR {
        EidmR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Extended ID Mask"]
    #[inline(always)]
    pub fn eidm(&mut self) -> EidmW<FdcanXidamSpec> {
        EidmW::new(self, 0)
    }
}
#[doc = "FDCAN Extended ID and Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_xidam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_xidam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanXidamSpec;
impl crate::RegisterSpec for FdcanXidamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_xidam::R`](R) reader structure"]
impl crate::Readable for FdcanXidamSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_xidam::W`](W) writer structure"]
impl crate::Writable for FdcanXidamSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_XIDAM to value 0"]
impl crate::Resettable for FdcanXidamSpec {}
