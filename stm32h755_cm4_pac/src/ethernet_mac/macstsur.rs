#[doc = "Register `MACSTSUR` reader"]
pub type R = crate::R<MacstsurSpec>;
#[doc = "Register `MACSTSUR` writer"]
pub type W = crate::W<MacstsurSpec>;
#[doc = "Field `TSS` reader - TSS"]
pub type TssR = crate::FieldReader<u32>;
#[doc = "Field `TSS` writer - TSS"]
pub type TssW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TSS"]
    #[inline(always)]
    pub fn tss(&self) -> TssR {
        TssR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSS"]
    #[inline(always)]
    pub fn tss(&mut self) -> TssW<MacstsurSpec> {
        TssW::new(self, 0)
    }
}
#[doc = "System time seconds update register\n\nYou can [`read`](crate::Reg::read) this register and get [`macstsur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macstsur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacstsurSpec;
impl crate::RegisterSpec for MacstsurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macstsur::R`](R) reader structure"]
impl crate::Readable for MacstsurSpec {}
#[doc = "`write(|w| ..)` method takes [`macstsur::W`](W) writer structure"]
impl crate::Writable for MacstsurSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACSTSUR to value 0"]
impl crate::Resettable for MacstsurSpec {}
