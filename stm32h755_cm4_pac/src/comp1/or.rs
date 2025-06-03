#[doc = "Register `OR` reader"]
pub type R = crate::R<OrSpec>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<OrSpec>;
#[doc = "Field `AFOP` reader - Selection of source for alternate function of output ports"]
pub type AfopR = crate::FieldReader<u16>;
#[doc = "Field `AFOP` writer - Selection of source for alternate function of output ports"]
pub type AfopW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `OR` reader - Option Register"]
pub type OrR = crate::FieldReader<u32>;
#[doc = "Field `OR` writer - Option Register"]
pub type OrW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:10 - Selection of source for alternate function of output ports"]
    #[inline(always)]
    pub fn afop(&self) -> AfopR {
        AfopR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - Option Register"]
    #[inline(always)]
    pub fn or(&self) -> OrR {
        OrR::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Selection of source for alternate function of output ports"]
    #[inline(always)]
    pub fn afop(&mut self) -> AfopW<OrSpec> {
        AfopW::new(self, 0)
    }
    #[doc = "Bits 11:31 - Option Register"]
    #[inline(always)]
    pub fn or(&mut self) -> OrW<OrSpec> {
        OrW::new(self, 11)
    }
}
#[doc = "Comparator option register\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OrSpec;
impl crate::RegisterSpec for OrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for OrSpec {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for OrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OrSpec {}
