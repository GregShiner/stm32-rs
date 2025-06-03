#[doc = "Register `MACPPSTTNR` reader"]
pub type R = crate::R<MacppsttnrSpec>;
#[doc = "Register `MACPPSTTNR` writer"]
pub type W = crate::W<MacppsttnrSpec>;
#[doc = "Field `TTSL0` reader - TTSL0"]
pub type Ttsl0R = crate::FieldReader<u32>;
#[doc = "Field `TTSL0` writer - TTSL0"]
pub type Ttsl0W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `TRGTBUSY0` reader - TRGTBUSY0"]
pub type Trgtbusy0R = crate::BitReader;
#[doc = "Field `TRGTBUSY0` writer - TRGTBUSY0"]
pub type Trgtbusy0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - TTSL0"]
    #[inline(always)]
    pub fn ttsl0(&self) -> Ttsl0R {
        Ttsl0R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - TRGTBUSY0"]
    #[inline(always)]
    pub fn trgtbusy0(&self) -> Trgtbusy0R {
        Trgtbusy0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - TTSL0"]
    #[inline(always)]
    pub fn ttsl0(&mut self) -> Ttsl0W<MacppsttnrSpec> {
        Ttsl0W::new(self, 0)
    }
    #[doc = "Bit 31 - TRGTBUSY0"]
    #[inline(always)]
    pub fn trgtbusy0(&mut self) -> Trgtbusy0W<MacppsttnrSpec> {
        Trgtbusy0W::new(self, 31)
    }
}
#[doc = "PPS target time nanoseconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`macppsttnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsttnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacppsttnrSpec;
impl crate::RegisterSpec for MacppsttnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppsttnr::R`](R) reader structure"]
impl crate::Readable for MacppsttnrSpec {}
#[doc = "`write(|w| ..)` method takes [`macppsttnr::W`](W) writer structure"]
impl crate::Writable for MacppsttnrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACPPSTTNR to value 0"]
impl crate::Resettable for MacppsttnrSpec {}
