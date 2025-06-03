#[doc = "Register `MACLMIR` reader"]
pub type R = crate::R<MaclmirSpec>;
#[doc = "Register `MACLMIR` writer"]
pub type W = crate::W<MaclmirSpec>;
#[doc = "Field `LSI` reader - LSI"]
pub type LsiR = crate::FieldReader;
#[doc = "Field `LSI` writer - LSI"]
pub type LsiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DRSYNCR` reader - DRSYNCR"]
pub type DrsyncrR = crate::FieldReader;
#[doc = "Field `DRSYNCR` writer - DRSYNCR"]
pub type DrsyncrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LMPDRI` reader - LMPDRI"]
pub type LmpdriR = crate::FieldReader;
#[doc = "Field `LMPDRI` writer - LMPDRI"]
pub type LmpdriW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - LSI"]
    #[inline(always)]
    pub fn lsi(&self) -> LsiR {
        LsiR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - DRSYNCR"]
    #[inline(always)]
    pub fn drsyncr(&self) -> DrsyncrR {
        DrsyncrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 24:31 - LMPDRI"]
    #[inline(always)]
    pub fn lmpdri(&self) -> LmpdriR {
        LmpdriR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LSI"]
    #[inline(always)]
    pub fn lsi(&mut self) -> LsiW<MaclmirSpec> {
        LsiW::new(self, 0)
    }
    #[doc = "Bits 8:10 - DRSYNCR"]
    #[inline(always)]
    pub fn drsyncr(&mut self) -> DrsyncrW<MaclmirSpec> {
        DrsyncrW::new(self, 8)
    }
    #[doc = "Bits 24:31 - LMPDRI"]
    #[inline(always)]
    pub fn lmpdri(&mut self) -> LmpdriW<MaclmirSpec> {
        LmpdriW::new(self, 24)
    }
}
#[doc = "Log message interval register\n\nYou can [`read`](crate::Reg::read) this register and get [`maclmir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclmir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaclmirSpec;
impl crate::RegisterSpec for MaclmirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maclmir::R`](R) reader structure"]
impl crate::Readable for MaclmirSpec {}
#[doc = "`write(|w| ..)` method takes [`maclmir::W`](W) writer structure"]
impl crate::Writable for MaclmirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACLMIR to value 0"]
impl crate::Resettable for MaclmirSpec {}
