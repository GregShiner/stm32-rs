#[doc = "Register `FDCAN_TOCV` reader"]
pub type R = crate::R<FdcanTocvSpec>;
#[doc = "Register `FDCAN_TOCV` writer"]
pub type W = crate::W<FdcanTocvSpec>;
#[doc = "Field `TOC` reader - Timeout Counter"]
pub type TocR = crate::FieldReader<u16>;
#[doc = "Field `TOC` writer - Timeout Counter"]
pub type TocW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timeout Counter"]
    #[inline(always)]
    pub fn toc(&self) -> TocR {
        TocR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout Counter"]
    #[inline(always)]
    pub fn toc(&mut self) -> TocW<FdcanTocvSpec> {
        TocW::new(self, 0)
    }
}
#[doc = "FDCAN Timeout Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tocv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTocvSpec;
impl crate::RegisterSpec for FdcanTocvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tocv::R`](R) reader structure"]
impl crate::Readable for FdcanTocvSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tocv::W`](W) writer structure"]
impl crate::Writable for FdcanTocvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TOCV to value 0"]
impl crate::Resettable for FdcanTocvSpec {}
