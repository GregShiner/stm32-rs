#[doc = "Register `UDRDR` reader"]
pub type R = crate::R<UdrdrSpec>;
#[doc = "Register `UDRDR` writer"]
pub type W = crate::W<UdrdrSpec>;
#[doc = "Field `UDRDR` reader - Data at slave underrun condition"]
pub type UdrdrR = crate::FieldReader<u32>;
#[doc = "Field `UDRDR` writer - Data at slave underrun condition"]
pub type UdrdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data at slave underrun condition"]
    #[inline(always)]
    pub fn udrdr(&self) -> UdrdrR {
        UdrdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data at slave underrun condition"]
    #[inline(always)]
    pub fn udrdr(&mut self) -> UdrdrW<UdrdrSpec> {
        UdrdrW::new(self, 0)
    }
}
#[doc = "Underrun Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`udrdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udrdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UdrdrSpec;
impl crate::RegisterSpec for UdrdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udrdr::R`](R) reader structure"]
impl crate::Readable for UdrdrSpec {}
#[doc = "`write(|w| ..)` method takes [`udrdr::W`](W) writer structure"]
impl crate::Writable for UdrdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDRDR to value 0"]
impl crate::Resettable for UdrdrSpec {}
