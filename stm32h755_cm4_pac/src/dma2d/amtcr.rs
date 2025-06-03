#[doc = "Register `AMTCR` reader"]
pub type R = crate::R<AmtcrSpec>;
#[doc = "Register `AMTCR` writer"]
pub type W = crate::W<AmtcrSpec>;
#[doc = "Field `EN` reader - Enable Enables the dead time functionality."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable Enables the dead time functionality."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT` reader - Dead Time Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses."]
pub type DtR = crate::FieldReader;
#[doc = "Field `DT` writer - Dead Time Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses."]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable Enables the dead time functionality."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Dead Time Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses."]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Enables the dead time functionality."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<AmtcrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Dead Time Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses."]
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<AmtcrSpec> {
        DtW::new(self, 8)
    }
}
#[doc = "DMA2D AXI master timer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`amtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmtcrSpec;
impl crate::RegisterSpec for AmtcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amtcr::R`](R) reader structure"]
impl crate::Readable for AmtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`amtcr::W`](W) writer structure"]
impl crate::Writable for AmtcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AMTCR to value 0"]
impl crate::Resettable for AmtcrSpec {}
