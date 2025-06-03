#[doc = "Register `MACMDIODR` reader"]
pub type R = crate::R<MacmdiodrSpec>;
#[doc = "Register `MACMDIODR` writer"]
pub type W = crate::W<MacmdiodrSpec>;
#[doc = "Field `MD` reader - MD"]
pub type MdR = crate::FieldReader<u16>;
#[doc = "Field `MD` writer - MD"]
pub type MdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RA` reader - RA"]
pub type RaR = crate::FieldReader<u16>;
#[doc = "Field `RA` writer - RA"]
pub type RaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MD"]
    #[inline(always)]
    pub fn md(&self) -> MdR {
        MdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - RA"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MD"]
    #[inline(always)]
    pub fn md(&mut self) -> MdW<MacmdiodrSpec> {
        MdW::new(self, 0)
    }
    #[doc = "Bits 16:31 - RA"]
    #[inline(always)]
    pub fn ra(&mut self) -> RaW<MacmdiodrSpec> {
        RaW::new(self, 16)
    }
}
#[doc = "MDIO data register\n\nYou can [`read`](crate::Reg::read) this register and get [`macmdiodr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmdiodr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacmdiodrSpec;
impl crate::RegisterSpec for MacmdiodrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmdiodr::R`](R) reader structure"]
impl crate::Readable for MacmdiodrSpec {}
#[doc = "`write(|w| ..)` method takes [`macmdiodr::W`](W) writer structure"]
impl crate::Writable for MacmdiodrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACMDIODR to value 0"]
impl crate::Resettable for MacmdiodrSpec {}
