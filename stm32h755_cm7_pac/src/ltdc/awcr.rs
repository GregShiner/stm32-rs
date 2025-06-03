#[doc = "Register `AWCR` reader"]
pub type R = crate::R<AwcrSpec>;
#[doc = "Register `AWCR` writer"]
pub type W = crate::W<AwcrSpec>;
#[doc = "Field `AAH` reader - Accumulated Active Height (in units of horizontal scan line)"]
pub type AahR = crate::FieldReader<u16>;
#[doc = "Field `AAH` writer - Accumulated Active Height (in units of horizontal scan line)"]
pub type AahW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `AAV` reader - AAV"]
pub type AavR = crate::FieldReader<u16>;
#[doc = "Field `AAV` writer - AAV"]
pub type AavW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn aah(&self) -> AahR {
        AahR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - AAV"]
    #[inline(always)]
    pub fn aav(&self) -> AavR {
        AavR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn aah(&mut self) -> AahW<AwcrSpec> {
        AahW::new(self, 0)
    }
    #[doc = "Bits 16:27 - AAV"]
    #[inline(always)]
    pub fn aav(&mut self) -> AavW<AwcrSpec> {
        AavW::new(self, 16)
    }
}
#[doc = "Active Width Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`awcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwcrSpec;
impl crate::RegisterSpec for AwcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awcr::R`](R) reader structure"]
impl crate::Readable for AwcrSpec {}
#[doc = "`write(|w| ..)` method takes [`awcr::W`](W) writer structure"]
impl crate::Writable for AwcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWCR to value 0"]
impl crate::Resettable for AwcrSpec {}
