#[doc = "Register `BMPER` reader"]
pub type R = crate::R<BmperSpec>;
#[doc = "Register `BMPER` writer"]
pub type W = crate::W<BmperSpec>;
#[doc = "Field `BMPER` reader - Burst mode Period"]
pub type BmperR = crate::FieldReader<u16>;
#[doc = "Field `BMPER` writer - Burst mode Period"]
pub type BmperW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Burst mode Period"]
    #[inline(always)]
    pub fn bmper(&self) -> BmperR {
        BmperR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Burst mode Period"]
    #[inline(always)]
    pub fn bmper(&mut self) -> BmperW<BmperSpec> {
        BmperW::new(self, 0)
    }
}
#[doc = "Burst Mode Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bmper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmperSpec;
impl crate::RegisterSpec for BmperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmper::R`](R) reader structure"]
impl crate::Readable for BmperSpec {}
#[doc = "`write(|w| ..)` method takes [`bmper::W`](W) writer structure"]
impl crate::Writable for BmperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMPER to value 0"]
impl crate::Resettable for BmperSpec {}
