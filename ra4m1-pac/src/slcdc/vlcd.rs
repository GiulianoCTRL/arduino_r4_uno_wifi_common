#[doc = "Register `VLCD` reader"]
pub type R = crate::R<VLCD_SPEC>;
#[doc = "Register `VLCD` writer"]
pub type W = crate::W<VLCD_SPEC>;
#[doc = "Field `VLCD` reader - Reference Voltage(Contrast Adjustment) Select"]
pub type VLCD_R = crate::FieldReader<VLCD_A>;
#[doc = "Reference Voltage(Contrast Adjustment) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VLCD_A {
    #[doc = "4: Reference voltageselection(contrast adjustment): 1.00 V (default) VL4 voltage: 3.00 V(1/3 bias method)/4.00 V(1/4 bias method)"]
    _00100 = 4,
    #[doc = "5: Reference voltageselection(contrast adjustment): 1.05 V VL4 voltage: 3.15 V(1/3 bias method)/4.20 V(1/4 bias method)"]
    _00101 = 5,
    #[doc = "6: Reference voltageselection(contrast adjustment): 1.10 V VL4 voltage: 3.30 V(1/3 bias method)/4.40 V(1/4 bias method)"]
    _00110 = 6,
    #[doc = "7: Reference voltageselection(contrast adjustment): 1.15 V VL4 voltage: 3.45 V(1/3 bias method)/4.60 V(1/4 bias method)"]
    _00111 = 7,
    #[doc = "8: Reference voltageselection(contrast adjustment): 1.20 V VL4 voltage: 3.60 V(1/3 bias method)/4.80 V(1/4 bias method)"]
    _01000 = 8,
    #[doc = "9: Reference voltageselection(contrast adjustment): 1.25 V VL4 voltage: 3.75 V(1/3 bias method)/5.00 V(1/4 bias method)"]
    _01001 = 9,
    #[doc = "10: Reference voltageselection(contrast adjustment): 1.30 V VL4 voltage: 3.90 V(1/3 bias method)/5.20 V(1/4 bias method)"]
    _01010 = 10,
    #[doc = "11: Reference voltageselection(contrast adjustment): 1.35 V VL4 voltage: 4.05 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _01011 = 11,
    #[doc = "12: Reference voltageselection(contrast adjustment): 1.40 V VL4 voltage: 4.20 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _01100 = 12,
    #[doc = "13: Reference voltageselection(contrast adjustment): 1.45 V VL4 voltage: 4.35 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _01101 = 13,
    #[doc = "14: Reference voltageselection(contrast adjustment): 1.50 V VL4 voltage: 4.50 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _01110 = 14,
    #[doc = "15: Reference voltageselection(contrast adjustment): 1.55 V VL4 voltage: 4.65 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _01111 = 15,
    #[doc = "16: Reference voltageselection(contrast adjustment): 1.60 V VL4 voltage: 4.80 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _10000 = 16,
    #[doc = "17: Reference voltageselection(contrast adjustment): 1.65 V VL4 voltage: 4.95 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _10001 = 17,
    #[doc = "18: Reference voltageselection(contrast adjustment): 1.70 V VL4 voltage: 5.10 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _10010 = 18,
    #[doc = "19: Reference voltageselection(contrast adjustment): 1.75 V VL4 voltage: 5.25 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    _10011 = 19,
}
impl From<VLCD_A> for u8 {
    #[inline(always)]
    fn from(variant: VLCD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VLCD_A {
    type Ux = u8;
}
impl VLCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VLCD_A> {
        match self.bits {
            4 => Some(VLCD_A::_00100),
            5 => Some(VLCD_A::_00101),
            6 => Some(VLCD_A::_00110),
            7 => Some(VLCD_A::_00111),
            8 => Some(VLCD_A::_01000),
            9 => Some(VLCD_A::_01001),
            10 => Some(VLCD_A::_01010),
            11 => Some(VLCD_A::_01011),
            12 => Some(VLCD_A::_01100),
            13 => Some(VLCD_A::_01101),
            14 => Some(VLCD_A::_01110),
            15 => Some(VLCD_A::_01111),
            16 => Some(VLCD_A::_10000),
            17 => Some(VLCD_A::_10001),
            18 => Some(VLCD_A::_10010),
            19 => Some(VLCD_A::_10011),
            _ => None,
        }
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.00 V (default) VL4 voltage: 3.00 V(1/3 bias method)/4.00 V(1/4 bias method)"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == VLCD_A::_00100
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.05 V VL4 voltage: 3.15 V(1/3 bias method)/4.20 V(1/4 bias method)"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == VLCD_A::_00101
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.10 V VL4 voltage: 3.30 V(1/3 bias method)/4.40 V(1/4 bias method)"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == VLCD_A::_00110
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.15 V VL4 voltage: 3.45 V(1/3 bias method)/4.60 V(1/4 bias method)"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == VLCD_A::_00111
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.20 V VL4 voltage: 3.60 V(1/3 bias method)/4.80 V(1/4 bias method)"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == VLCD_A::_01000
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.25 V VL4 voltage: 3.75 V(1/3 bias method)/5.00 V(1/4 bias method)"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == VLCD_A::_01001
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.30 V VL4 voltage: 3.90 V(1/3 bias method)/5.20 V(1/4 bias method)"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == VLCD_A::_01010
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.35 V VL4 voltage: 4.05 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == VLCD_A::_01011
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.40 V VL4 voltage: 4.20 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == VLCD_A::_01100
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.45 V VL4 voltage: 4.35 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == VLCD_A::_01101
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.50 V VL4 voltage: 4.50 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == VLCD_A::_01110
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.55 V VL4 voltage: 4.65 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == VLCD_A::_01111
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.60 V VL4 voltage: 4.80 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == VLCD_A::_10000
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.65 V VL4 voltage: 4.95 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == VLCD_A::_10001
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.70 V VL4 voltage: 5.10 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == VLCD_A::_10010
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.75 V VL4 voltage: 5.25 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == VLCD_A::_10011
    }
}
#[doc = "Field `VLCD` writer - Reference Voltage(Contrast Adjustment) Select"]
pub type VLCD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, VLCD_A>;
impl<'a, REG, const O: u8> VLCD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reference voltageselection(contrast adjustment): 1.00 V (default) VL4 voltage: 3.00 V(1/3 bias method)/4.00 V(1/4 bias method)"]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_00100)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.05 V VL4 voltage: 3.15 V(1/3 bias method)/4.20 V(1/4 bias method)"]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_00101)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.10 V VL4 voltage: 3.30 V(1/3 bias method)/4.40 V(1/4 bias method)"]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_00110)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.15 V VL4 voltage: 3.45 V(1/3 bias method)/4.60 V(1/4 bias method)"]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_00111)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.20 V VL4 voltage: 3.60 V(1/3 bias method)/4.80 V(1/4 bias method)"]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01000)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.25 V VL4 voltage: 3.75 V(1/3 bias method)/5.00 V(1/4 bias method)"]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01001)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.30 V VL4 voltage: 3.90 V(1/3 bias method)/5.20 V(1/4 bias method)"]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01010)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.35 V VL4 voltage: 4.05 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01011)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.40 V VL4 voltage: 4.20 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01100)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.45 V VL4 voltage: 4.35 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01101)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.50 V VL4 voltage: 4.50 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01110)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.55 V VL4 voltage: 4.65 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01111)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.60 V VL4 voltage: 4.80 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_10000)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.65 V VL4 voltage: 4.95 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_10001)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.70 V VL4 voltage: 5.10 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_10010)
    }
    #[doc = "Reference voltageselection(contrast adjustment): 1.75 V VL4 voltage: 5.25 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_10011)
    }
}
impl R {
    #[doc = "Bits 0:4 - Reference Voltage(Contrast Adjustment) Select"]
    #[inline(always)]
    pub fn vlcd(&self) -> VLCD_R {
        VLCD_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reference Voltage(Contrast Adjustment) Select"]
    #[inline(always)]
    #[must_use]
    pub fn vlcd(&mut self) -> VLCD_W<VLCD_SPEC, 0> {
        VLCD_W::new(self)
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
#[doc = "LCD Boost Level Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlcd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlcd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VLCD_SPEC;
impl crate::RegisterSpec for VLCD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vlcd::R`](R) reader structure"]
impl crate::Readable for VLCD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vlcd::W`](W) writer structure"]
impl crate::Writable for VLCD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VLCD to value 0x04"]
impl crate::Resettable for VLCD_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
