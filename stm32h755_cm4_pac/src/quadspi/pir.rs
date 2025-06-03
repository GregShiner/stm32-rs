#[doc = "Register `PIR` reader"]
pub type R = crate::R<PirSpec>;
#[doc = "Register `PIR` writer"]
pub type W = crate::W<PirSpec>;
#[doc = "Field `INTERVAL` reader - Polling interval Number of CLK cycles between to read during automatic polling phases. This field can be written only when BUSY = 0."]
pub type IntervalR = crate::FieldReader<u16>;
#[doc = "Field `INTERVAL` writer - Polling interval Number of CLK cycles between to read during automatic polling phases. This field can be written only when BUSY = 0."]
pub type IntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Polling interval Number of CLK cycles between to read during automatic polling phases. This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn interval(&self) -> IntervalR {
        IntervalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Polling interval Number of CLK cycles between to read during automatic polling phases. This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn interval(&mut self) -> IntervalW<PirSpec> {
        IntervalW::new(self, 0)
    }
}
#[doc = "QUADSPI polling interval register\n\nYou can [`read`](crate::Reg::read) this register and get [`pir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PirSpec;
impl crate::RegisterSpec for PirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pir::R`](R) reader structure"]
impl crate::Readable for PirSpec {}
#[doc = "`write(|w| ..)` method takes [`pir::W`](W) writer structure"]
impl crate::Writable for PirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIR to value 0"]
impl crate::Resettable for PirSpec {}
