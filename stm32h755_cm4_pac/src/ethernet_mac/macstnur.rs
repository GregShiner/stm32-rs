#[doc = "Register `MACSTNUR` reader"]
pub type R = crate::R<MacstnurSpec>;
#[doc = "Register `MACSTNUR` writer"]
pub type W = crate::W<MacstnurSpec>;
#[doc = "Field `TSSS` reader - TSSS"]
pub type TsssR = crate::FieldReader<u32>;
#[doc = "Field `TSSS` writer - TSSS"]
pub type TsssW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `ADDSUB` reader - ADDSUB"]
pub type AddsubR = crate::BitReader;
#[doc = "Field `ADDSUB` writer - ADDSUB"]
pub type AddsubW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - TSSS"]
    #[inline(always)]
    pub fn tsss(&self) -> TsssR {
        TsssR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - ADDSUB"]
    #[inline(always)]
    pub fn addsub(&self) -> AddsubR {
        AddsubR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - TSSS"]
    #[inline(always)]
    pub fn tsss(&mut self) -> TsssW<MacstnurSpec> {
        TsssW::new(self, 0)
    }
    #[doc = "Bit 31 - ADDSUB"]
    #[inline(always)]
    pub fn addsub(&mut self) -> AddsubW<MacstnurSpec> {
        AddsubW::new(self, 31)
    }
}
#[doc = "System time nanoseconds update register\n\nYou can [`read`](crate::Reg::read) this register and get [`macstnur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macstnur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacstnurSpec;
impl crate::RegisterSpec for MacstnurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macstnur::R`](R) reader structure"]
impl crate::Readable for MacstnurSpec {}
#[doc = "`write(|w| ..)` method takes [`macstnur::W`](W) writer structure"]
impl crate::Writable for MacstnurSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACSTNUR to value 0"]
impl crate::Resettable for MacstnurSpec {}
