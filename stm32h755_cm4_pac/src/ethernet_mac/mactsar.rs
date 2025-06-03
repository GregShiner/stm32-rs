#[doc = "Register `MACTSAR` reader"]
pub type R = crate::R<MactsarSpec>;
#[doc = "Register `MACTSAR` writer"]
pub type W = crate::W<MactsarSpec>;
#[doc = "Field `TSAR` reader - TSAR"]
pub type TsarR = crate::FieldReader<u32>;
#[doc = "Field `TSAR` writer - TSAR"]
pub type TsarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TSAR"]
    #[inline(always)]
    pub fn tsar(&self) -> TsarR {
        TsarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSAR"]
    #[inline(always)]
    pub fn tsar(&mut self) -> TsarW<MactsarSpec> {
        TsarW::new(self, 0)
    }
}
#[doc = "Timestamp addend register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MactsarSpec;
impl crate::RegisterSpec for MactsarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactsar::R`](R) reader structure"]
impl crate::Readable for MactsarSpec {}
#[doc = "`write(|w| ..)` method takes [`mactsar::W`](W) writer structure"]
impl crate::Writable for MactsarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACTSAR to value 0"]
impl crate::Resettable for MactsarSpec {}
