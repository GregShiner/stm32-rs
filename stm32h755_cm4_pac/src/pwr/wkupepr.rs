#[doc = "Register `WKUPEPR` reader"]
pub type R = crate::R<WkupeprSpec>;
#[doc = "Register `WKUPEPR` writer"]
pub type W = crate::W<WkupeprSpec>;
#[doc = "Field `WKUPEN1` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type Wkupen1R = crate::BitReader;
#[doc = "Field `WKUPEN1` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type Wkupen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEN2` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type Wkupen2R = crate::BitReader;
#[doc = "Field `WKUPEN2` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type Wkupen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEN3` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type Wkupen3R = crate::BitReader;
#[doc = "Field `WKUPEN3` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type Wkupen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEN4` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type Wkupen4R = crate::BitReader;
#[doc = "Field `WKUPEN4` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type Wkupen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEN5` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type Wkupen5R = crate::BitReader;
#[doc = "Field `WKUPEN5` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type Wkupen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEN6` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type Wkupen6R = crate::BitReader;
#[doc = "Field `WKUPEN6` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
pub type Wkupen6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPP1` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type Wkupp1R = crate::BitReader;
#[doc = "Field `WKUPP1` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type Wkupp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPP2` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type Wkupp2R = crate::BitReader;
#[doc = "Field `WKUPP2` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type Wkupp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPP3` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type Wkupp3R = crate::BitReader;
#[doc = "Field `WKUPP3` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type Wkupp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPP4` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type Wkupp4R = crate::BitReader;
#[doc = "Field `WKUPP4` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type Wkupp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPP5` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type Wkupp5R = crate::BitReader;
#[doc = "Field `WKUPP5` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type Wkupp5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPP6` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type Wkupp6R = crate::BitReader;
#[doc = "Field `WKUPP6` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
pub type Wkupp6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPPUPD1` reader - Wakeup pin pull configuration"]
pub type Wkuppupd1R = crate::FieldReader;
#[doc = "Field `WKUPPUPD1` writer - Wakeup pin pull configuration"]
pub type Wkuppupd1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WKUPPUPD2` reader - Wakeup pin pull configuration"]
pub type Wkuppupd2R = crate::FieldReader;
#[doc = "Field `WKUPPUPD2` writer - Wakeup pin pull configuration"]
pub type Wkuppupd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WKUPPUPD3` reader - Wakeup pin pull configuration"]
pub type Wkuppupd3R = crate::FieldReader;
#[doc = "Field `WKUPPUPD3` writer - Wakeup pin pull configuration"]
pub type Wkuppupd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WKUPPUPD4` reader - Wakeup pin pull configuration"]
pub type Wkuppupd4R = crate::FieldReader;
#[doc = "Field `WKUPPUPD4` writer - Wakeup pin pull configuration"]
pub type Wkuppupd4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WKUPPUPD5` reader - Wakeup pin pull configuration"]
pub type Wkuppupd5R = crate::FieldReader;
#[doc = "Field `WKUPPUPD5` writer - Wakeup pin pull configuration"]
pub type Wkuppupd5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WKUPPUPD6` reader - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode."]
pub type Wkuppupd6R = crate::FieldReader;
#[doc = "Field `WKUPPUPD6` writer - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode."]
pub type Wkuppupd6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen1(&self) -> Wkupen1R {
        Wkupen1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen2(&self) -> Wkupen2R {
        Wkupen2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen3(&self) -> Wkupen3R {
        Wkupen3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen4(&self) -> Wkupen4R {
        Wkupen4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen5(&self) -> Wkupen5R {
        Wkupen5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen6(&self) -> Wkupen6R {
        Wkupen6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp1(&self) -> Wkupp1R {
        Wkupp1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp2(&self) -> Wkupp2R {
        Wkupp2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp3(&self) -> Wkupp3R {
        Wkupp3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp4(&self) -> Wkupp4R {
        Wkupp4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp5(&self) -> Wkupp5R {
        Wkupp5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp6(&self) -> Wkupp6R {
        Wkupp6R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd1(&self) -> Wkuppupd1R {
        Wkuppupd1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd2(&self) -> Wkuppupd2R {
        Wkuppupd2R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd3(&self) -> Wkuppupd3R {
        Wkuppupd3R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd4(&self) -> Wkuppupd4R {
        Wkuppupd4R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd5(&self) -> Wkuppupd5R {
        Wkuppupd5R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn wkuppupd6(&self) -> Wkuppupd6R {
        Wkuppupd6R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen1(&mut self) -> Wkupen1W<WkupeprSpec> {
        Wkupen1W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen2(&mut self) -> Wkupen2W<WkupeprSpec> {
        Wkupen2W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen3(&mut self) -> Wkupen3W<WkupeprSpec> {
        Wkupen3W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen4(&mut self) -> Wkupen4W<WkupeprSpec> {
        Wkupen4W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen5(&mut self) -> Wkupen5W<WkupeprSpec> {
        Wkupen5W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge."]
    #[inline(always)]
    pub fn wkupen6(&mut self) -> Wkupen6W<WkupeprSpec> {
        Wkupen6W::new(self, 5)
    }
    #[doc = "Bit 8 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp1(&mut self) -> Wkupp1W<WkupeprSpec> {
        Wkupp1W::new(self, 8)
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp2(&mut self) -> Wkupp2W<WkupeprSpec> {
        Wkupp2W::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp3(&mut self) -> Wkupp3W<WkupeprSpec> {
        Wkupp3W::new(self, 10)
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp4(&mut self) -> Wkupp4W<WkupeprSpec> {
        Wkupp4W::new(self, 11)
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp5(&mut self) -> Wkupp5W<WkupeprSpec> {
        Wkupp5W::new(self, 12)
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin."]
    #[inline(always)]
    pub fn wkupp6(&mut self) -> Wkupp6W<WkupeprSpec> {
        Wkupp6W::new(self, 13)
    }
    #[doc = "Bits 16:17 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd1(&mut self) -> Wkuppupd1W<WkupeprSpec> {
        Wkuppupd1W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd2(&mut self) -> Wkuppupd2W<WkupeprSpec> {
        Wkuppupd2W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd3(&mut self) -> Wkuppupd3W<WkupeprSpec> {
        Wkuppupd3W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd4(&mut self) -> Wkuppupd4W<WkupeprSpec> {
        Wkuppupd4W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Wakeup pin pull configuration"]
    #[inline(always)]
    pub fn wkuppupd5(&mut self) -> Wkuppupd5W<WkupeprSpec> {
        Wkuppupd5W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn wkuppupd6(&mut self) -> Wkuppupd6W<WkupeprSpec> {
        Wkuppupd6W::new(self, 26)
    }
}
#[doc = "Reset only by system reset, not reset by wakeup from Standby mode\n\nYou can [`read`](crate::Reg::read) this register and get [`wkupepr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupepr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkupeprSpec;
impl crate::RegisterSpec for WkupeprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkupepr::R`](R) reader structure"]
impl crate::Readable for WkupeprSpec {}
#[doc = "`write(|w| ..)` method takes [`wkupepr::W`](W) writer structure"]
impl crate::Writable for WkupeprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WKUPEPR to value 0"]
impl crate::Resettable for WkupeprSpec {}
