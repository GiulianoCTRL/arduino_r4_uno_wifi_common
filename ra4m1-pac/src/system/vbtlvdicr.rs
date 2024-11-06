#[doc = "Register `VBTLVDICR` reader"]
pub type R = crate::R<VBTLVDICR_SPEC>;
#[doc = "Register `VBTLVDICR` writer"]
pub type W = crate::W<VBTLVDICR_SPEC>;
#[doc = "Field `VBTLVDIE` reader - VBATT Pin Low Voltage Detect Interrupt Enable bit"]
pub type VBTLVDIE_R = crate::BitReader<VBTLVDIE_A>;
#[doc = "VBATT Pin Low Voltage Detect Interrupt Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTLVDIE_A {
    #[doc = "0: VBATT Pin Low Voltage Detect Interrupt Disable"]
    _0 = 0,
    #[doc = "1: VBATT Pin Low Voltage Detect Interrupt Enable"]
    _1 = 1,
}
impl From<VBTLVDIE_A> for bool {
    #[inline(always)]
    fn from(variant: VBTLVDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl VBTLVDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBTLVDIE_A {
        match self.bits {
            false => VBTLVDIE_A::_0,
            true => VBTLVDIE_A::_1,
        }
    }
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTLVDIE_A::_0
    }
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTLVDIE_A::_1
    }
}
#[doc = "Field `VBTLVDIE` writer - VBATT Pin Low Voltage Detect Interrupt Enable bit"]
pub type VBTLVDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VBTLVDIE_A>;
impl<'a, REG, const O: u8> VBTLVDIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDIE_A::_0)
    }
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDIE_A::_1)
    }
}
#[doc = "Field `VBTLVDISEL` reader - Pin Low Voltage Detect Interrupt Select bit"]
pub type VBTLVDISEL_R = crate::BitReader<VBTLVDISEL_A>;
#[doc = "Pin Low Voltage Detect Interrupt Select bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTLVDISEL_A {
    #[doc = "0: Non Maskable Interrupt"]
    _0 = 0,
    #[doc = "1: Maskable Interrupt"]
    _1 = 1,
}
impl From<VBTLVDISEL_A> for bool {
    #[inline(always)]
    fn from(variant: VBTLVDISEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VBTLVDISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBTLVDISEL_A {
        match self.bits {
            false => VBTLVDISEL_A::_0,
            true => VBTLVDISEL_A::_1,
        }
    }
    #[doc = "Non Maskable Interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTLVDISEL_A::_0
    }
    #[doc = "Maskable Interrupt"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTLVDISEL_A::_1
    }
}
#[doc = "Field `VBTLVDISEL` writer - Pin Low Voltage Detect Interrupt Select bit"]
pub type VBTLVDISEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VBTLVDISEL_A>;
impl<'a, REG, const O: u8> VBTLVDISEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non Maskable Interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDISEL_A::_0)
    }
    #[doc = "Maskable Interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDISEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT Pin Low Voltage Detect Interrupt Enable bit"]
    #[inline(always)]
    pub fn vbtlvdie(&self) -> VBTLVDIE_R {
        VBTLVDIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin Low Voltage Detect Interrupt Select bit"]
    #[inline(always)]
    pub fn vbtlvdisel(&self) -> VBTLVDISEL_R {
        VBTLVDISEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATT Pin Low Voltage Detect Interrupt Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn vbtlvdie(&mut self) -> VBTLVDIE_W<VBTLVDICR_SPEC, 0> {
        VBTLVDIE_W::new(self)
    }
    #[doc = "Bit 1 - Pin Low Voltage Detect Interrupt Select bit"]
    #[inline(always)]
    #[must_use]
    pub fn vbtlvdisel(&mut self) -> VBTLVDISEL_W<VBTLVDICR_SPEC, 1> {
        VBTLVDISEL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "VBATT Pin Low Voltage Detect Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtlvdicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtlvdicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBTLVDICR_SPEC;
impl crate::RegisterSpec for VBTLVDICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtlvdicr::R`](R) reader structure"]
impl crate::Readable for VBTLVDICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vbtlvdicr::W`](W) writer structure"]
impl crate::Writable for VBTLVDICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTLVDICR to value 0"]
impl crate::Resettable for VBTLVDICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
