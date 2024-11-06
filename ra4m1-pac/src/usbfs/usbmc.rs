#[doc = "Register `USBMC` reader"]
pub type R = crate::R<USBMC_SPEC>;
#[doc = "Register `USBMC` writer"]
pub type W = crate::W<USBMC_SPEC>;
#[doc = "Field `VDDUSBE` reader - USB Reference Power Supply Circuit On/Off Control"]
pub type VDDUSBE_R = crate::BitReader<VDDUSBE_A>;
#[doc = "USB Reference Power Supply Circuit On/Off Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDUSBE_A {
    #[doc = "0: USB reference power supply circuit off"]
    _0 = 0,
    #[doc = "1: USB reference power supply circuit on"]
    _1 = 1,
}
impl From<VDDUSBE_A> for bool {
    #[inline(always)]
    fn from(variant: VDDUSBE_A) -> Self {
        variant as u8 != 0
    }
}
impl VDDUSBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VDDUSBE_A {
        match self.bits {
            false => VDDUSBE_A::_0,
            true => VDDUSBE_A::_1,
        }
    }
    #[doc = "USB reference power supply circuit off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDDUSBE_A::_0
    }
    #[doc = "USB reference power supply circuit on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDDUSBE_A::_1
    }
}
#[doc = "Field `VDDUSBE` writer - USB Reference Power Supply Circuit On/Off Control"]
pub type VDDUSBE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VDDUSBE_A>;
impl<'a, REG, const O: u8> VDDUSBE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB reference power supply circuit off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VDDUSBE_A::_0)
    }
    #[doc = "USB reference power supply circuit on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VDDUSBE_A::_1)
    }
}
#[doc = "Field `VDCEN` reader - USB Regulator On/Off Control"]
pub type VDCEN_R = crate::BitReader<VDCEN_A>;
#[doc = "USB Regulator On/Off Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDCEN_A {
    #[doc = "0: USB regulator off"]
    _0 = 0,
    #[doc = "1: USB regulator on"]
    _1 = 1,
}
impl From<VDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: VDCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VDCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VDCEN_A {
        match self.bits {
            false => VDCEN_A::_0,
            true => VDCEN_A::_1,
        }
    }
    #[doc = "USB regulator off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDCEN_A::_0
    }
    #[doc = "USB regulator on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDCEN_A::_1
    }
}
#[doc = "Field `VDCEN` writer - USB Regulator On/Off Control"]
pub type VDCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VDCEN_A>;
impl<'a, REG, const O: u8> VDCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB regulator off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VDCEN_A::_0)
    }
    #[doc = "USB regulator on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VDCEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Reference Power Supply Circuit On/Off Control"]
    #[inline(always)]
    pub fn vddusbe(&self) -> VDDUSBE_R {
        VDDUSBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - USB Regulator On/Off Control"]
    #[inline(always)]
    pub fn vdcen(&self) -> VDCEN_R {
        VDCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Reference Power Supply Circuit On/Off Control"]
    #[inline(always)]
    #[must_use]
    pub fn vddusbe(&mut self) -> VDDUSBE_W<USBMC_SPEC, 0> {
        VDDUSBE_W::new(self)
    }
    #[doc = "Bit 7 - USB Regulator On/Off Control"]
    #[inline(always)]
    #[must_use]
    pub fn vdcen(&mut self) -> VDCEN_W<USBMC_SPEC, 7> {
        VDCEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Module Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbmc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbmc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBMC_SPEC;
impl crate::RegisterSpec for USBMC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbmc::R`](R) reader structure"]
impl crate::Readable for USBMC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbmc::W`](W) writer structure"]
impl crate::Writable for USBMC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBMC to value 0x02"]
impl crate::Resettable for USBMC_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
