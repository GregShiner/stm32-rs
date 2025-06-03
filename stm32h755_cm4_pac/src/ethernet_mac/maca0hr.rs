#[doc = "Register `MACA0HR` reader"]
pub type R = crate::R<Maca0hrSpec>;
#[doc = "Register `MACA0HR` writer"]
pub type W = crate::W<Maca0hrSpec>;
#[doc = "Field `ADDRHI` reader - ADDRHI"]
pub type AddrhiR = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI` writer - ADDRHI"]
pub type AddrhiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AE` reader - AE"]
pub type AeR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - ADDRHI"]
    #[inline(always)]
    pub fn addrhi(&self) -> AddrhiR {
        AddrhiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    pub fn ae(&self) -> AeR {
        AeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDRHI"]
    #[inline(always)]
    pub fn addrhi(&mut self) -> AddrhiW<Maca0hrSpec> {
        AddrhiW::new(self, 0)
    }
}
#[doc = "Address 0 high register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca0hr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0hr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca0hrSpec;
impl crate::RegisterSpec for Maca0hrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca0hr::R`](R) reader structure"]
impl crate::Readable for Maca0hrSpec {}
#[doc = "`write(|w| ..)` method takes [`maca0hr::W`](W) writer structure"]
impl crate::Writable for Maca0hrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACA0HR to value 0x8000_ffff"]
impl crate::Resettable for Maca0hrSpec {
    const RESET_VALUE: u32 = 0x8000_ffff;
}
