#[doc = "Register `CRCPOLY` reader"]
pub type R = crate::R<CrcpolySpec>;
#[doc = "Register `CRCPOLY` writer"]
pub type W = crate::W<CrcpolySpec>;
#[doc = "Field `CRCPOLY` reader - CRC polynomial register"]
pub type CrcpolyR = crate::FieldReader<u32>;
#[doc = "Field `CRCPOLY` writer - CRC polynomial register"]
pub type CrcpolyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC polynomial register"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CrcpolyR {
        CrcpolyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC polynomial register"]
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CrcpolyW<CrcpolySpec> {
        CrcpolyW::new(self, 0)
    }
}
#[doc = "Polynomial Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpoly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcpolySpec;
impl crate::RegisterSpec for CrcpolySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcpoly::R`](R) reader structure"]
impl crate::Readable for CrcpolySpec {}
#[doc = "`write(|w| ..)` method takes [`crcpoly::W`](W) writer structure"]
impl crate::Writable for CrcpolySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCPOLY to value 0x0107"]
impl crate::Resettable for CrcpolySpec {
    const RESET_VALUE: u32 = 0x0107;
}
