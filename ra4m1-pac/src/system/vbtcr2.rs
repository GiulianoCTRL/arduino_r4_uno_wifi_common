#[doc = "Register `VBTCR2` reader"]
pub type R = crate::R<VBTCR2_SPEC>;
#[doc = "Register `VBTCR2` writer"]
pub type W = crate::W<VBTCR2_SPEC>;
#[doc = "Field `VBTLVDEN` reader - VBATT Pin Low Voltage Detect Enable Bit"]
pub type VBTLVDEN_R = crate::BitReader<VBTLVDEN_A>;
#[doc = "VBATT Pin Low Voltage Detect Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTLVDEN_A {
    #[doc = "0: VBATT pin low voltage detect disable"]
    _0 = 0,
    #[doc = "1: VBATT pin low voltage detect enable"]
    _1 = 1,
}
impl From<VBTLVDEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBTLVDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VBTLVDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBTLVDEN_A {
        match self.bits {
            false => VBTLVDEN_A::_0,
            true => VBTLVDEN_A::_1,
        }
    }
    #[doc = "VBATT pin low voltage detect disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTLVDEN_A::_0
    }
    #[doc = "VBATT pin low voltage detect enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTLVDEN_A::_1
    }
}
#[doc = "Field `VBTLVDEN` writer - VBATT Pin Low Voltage Detect Enable Bit"]
pub type VBTLVDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VBTLVDEN_A>;
impl<'a, REG, const O: u8> VBTLVDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT pin low voltage detect disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDEN_A::_0)
    }
    #[doc = "VBATT pin low voltage detect enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDEN_A::_1)
    }
}
#[doc = "Field `VBTLVDLVL` reader - VBATT Pin Voltage Low Voltage Detect Level Select Bit"]
pub type VBTLVDLVL_R = crate::FieldReader<VBTLVDLVL_A>;
#[doc = "VBATT Pin Voltage Low Voltage Detect Level Select Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VBTLVDLVL_A {
    #[doc = "0: 2.7V"]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: 2.3V"]
    _10 = 2,
    #[doc = "3: 2.1V"]
    _11 = 3,
}
impl From<VBTLVDLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: VBTLVDLVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VBTLVDLVL_A {
    type Ux = u8;
}
impl VBTLVDLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBTLVDLVL_A {
        match self.bits {
            0 => VBTLVDLVL_A::_00,
            1 => VBTLVDLVL_A::_01,
            2 => VBTLVDLVL_A::_10,
            3 => VBTLVDLVL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "2.7V"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == VBTLVDLVL_A::_00
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == VBTLVDLVL_A::_01
    }
    #[doc = "2.3V"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == VBTLVDLVL_A::_10
    }
    #[doc = "2.1V"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == VBTLVDLVL_A::_11
    }
}
#[doc = "Field `VBTLVDLVL` writer - VBATT Pin Voltage Low Voltage Detect Level Select Bit"]
pub type VBTLVDLVL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, VBTLVDLVL_A>;
impl<'a, REG, const O: u8> VBTLVDLVL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.7V"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDLVL_A::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDLVL_A::_01)
    }
    #[doc = "2.3V"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDLVL_A::_10)
    }
    #[doc = "2.1V"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDLVL_A::_11)
    }
}
impl R {
    #[doc = "Bit 4 - VBATT Pin Low Voltage Detect Enable Bit"]
    #[inline(always)]
    pub fn vbtlvden(&self) -> VBTLVDEN_R {
        VBTLVDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - VBATT Pin Voltage Low Voltage Detect Level Select Bit"]
    #[inline(always)]
    pub fn vbtlvdlvl(&self) -> VBTLVDLVL_R {
        VBTLVDLVL_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 4 - VBATT Pin Low Voltage Detect Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn vbtlvden(&mut self) -> VBTLVDEN_W<VBTCR2_SPEC, 4> {
        VBTLVDEN_W::new(self)
    }
    #[doc = "Bits 6:7 - VBATT Pin Voltage Low Voltage Detect Level Select Bit"]
    #[inline(always)]
    #[must_use]
    pub fn vbtlvdlvl(&mut self) -> VBTLVDLVL_W<VBTCR2_SPEC, 6> {
        VBTLVDLVL_W::new(self)
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
#[doc = "VBATT Control Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBTCR2_SPEC;
impl crate::RegisterSpec for VBTCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtcr2::R`](R) reader structure"]
impl crate::Readable for VBTCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vbtcr2::W`](W) writer structure"]
impl crate::Writable for VBTCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTCR2 to value 0"]
impl crate::Resettable for VBTCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
