#[doc = "Register `DSI_GPDR` reader"]
pub type R = crate::R<DsiGpdrSpec>;
#[doc = "Register `DSI_GPDR` writer"]
pub type W = crate::W<DsiGpdrSpec>;
#[doc = "Field `DATA1` reader - DATA1"]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA1` writer - DATA1"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA2` reader - DATA2"]
pub type Data2R = crate::FieldReader;
#[doc = "Field `DATA2` writer - DATA2"]
pub type Data2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA3` reader - DATA3"]
pub type Data3R = crate::FieldReader;
#[doc = "Field `DATA3` writer - DATA3"]
pub type Data3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA4` reader - DATA4"]
pub type Data4R = crate::FieldReader;
#[doc = "Field `DATA4` writer - DATA4"]
pub type Data4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> Data4R {
        Data4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA1"]
    #[inline(always)]
    pub fn data1(&mut self) -> Data1W<DsiGpdrSpec> {
        Data1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DATA2"]
    #[inline(always)]
    pub fn data2(&mut self) -> Data2W<DsiGpdrSpec> {
        Data2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DATA3"]
    #[inline(always)]
    pub fn data3(&mut self) -> Data3W<DsiGpdrSpec> {
        Data3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DATA4"]
    #[inline(always)]
    pub fn data4(&mut self) -> Data4W<DsiGpdrSpec> {
        Data4W::new(self, 24)
    }
}
#[doc = "DSI Host generic payload data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_gpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_gpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiGpdrSpec;
impl crate::RegisterSpec for DsiGpdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_gpdr::R`](R) reader structure"]
impl crate::Readable for DsiGpdrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_gpdr::W`](W) writer structure"]
impl crate::Writable for DsiGpdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_GPDR to value 0"]
impl crate::Resettable for DsiGpdrSpec {}
