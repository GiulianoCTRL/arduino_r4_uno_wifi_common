#[doc = "Register `LVD%sCR1` reader"]
pub type R = crate::R<LVDCR1_SPEC>;
#[doc = "Register `LVD%sCR1` writer"]
pub type W = crate::W<LVDCR1_SPEC>;
#[doc = "Field `IDTSEL` reader - Voltage Monitor Interrupt Generation Condition Select"]
pub type IDTSEL_R = crate::FieldReader<IDTSEL_A>;
#[doc = "Voltage Monitor Interrupt Generation Condition Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDTSEL_A {
    #[doc = "0: When VCC>=Vdet (rise) is detected"]
    _00 = 0,
    #[doc = "1: When VCC&lt;Vdet (drop) is detected"]
    _01 = 1,
    #[doc = "2: When drop and rise are detected"]
    _10 = 2,
    #[doc = "3: Settings prohibited"]
    _11 = 3,
}
impl From<IDTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IDTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDTSEL_A {
    type Ux = u8;
}
impl IDTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDTSEL_A {
        match self.bits {
            0 => IDTSEL_A::_00,
            1 => IDTSEL_A::_01,
            2 => IDTSEL_A::_10,
            3 => IDTSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "When VCC>=Vdet (rise) is detected"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IDTSEL_A::_00
    }
    #[doc = "When VCC&lt;Vdet (drop) is detected"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IDTSEL_A::_01
    }
    #[doc = "When drop and rise are detected"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IDTSEL_A::_10
    }
    #[doc = "Settings prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IDTSEL_A::_11
    }
}
#[doc = "Field `IDTSEL` writer - Voltage Monitor Interrupt Generation Condition Select"]
pub type IDTSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, IDTSEL_A>;
impl<'a, REG, const O: u8> IDTSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When VCC>=Vdet (rise) is detected"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(IDTSEL_A::_00)
    }
    #[doc = "When VCC&lt;Vdet (drop) is detected"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(IDTSEL_A::_01)
    }
    #[doc = "When drop and rise are detected"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(IDTSEL_A::_10)
    }
    #[doc = "Settings prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(IDTSEL_A::_11)
    }
}
#[doc = "Field `IRQSEL` reader - Voltage Monitor Interrupt Type Select"]
pub type IRQSEL_R = crate::BitReader<IRQSEL_A>;
#[doc = "Voltage Monitor Interrupt Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQSEL_A {
    #[doc = "0: Non-maskable interrupt"]
    _0 = 0,
    #[doc = "1: Maskable interrupt"]
    _1 = 1,
}
impl From<IRQSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IRQSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IRQSEL_A {
        match self.bits {
            false => IRQSEL_A::_0,
            true => IRQSEL_A::_1,
        }
    }
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQSEL_A::_0
    }
    #[doc = "Maskable interrupt"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQSEL_A::_1
    }
}
#[doc = "Field `IRQSEL` writer - Voltage Monitor Interrupt Type Select"]
pub type IRQSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRQSEL_A>;
impl<'a, REG, const O: u8> IRQSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQSEL_A::_0)
    }
    #[doc = "Maskable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQSEL_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Voltage Monitor Interrupt Generation Condition Select"]
    #[inline(always)]
    pub fn idtsel(&self) -> IDTSEL_R {
        IDTSEL_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Voltage Monitor Interrupt Type Select"]
    #[inline(always)]
    pub fn irqsel(&self) -> IRQSEL_R {
        IRQSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Voltage Monitor Interrupt Generation Condition Select"]
    #[inline(always)]
    #[must_use]
    pub fn idtsel(&mut self) -> IDTSEL_W<LVDCR1_SPEC, 0> {
        IDTSEL_W::new(self)
    }
    #[doc = "Bit 2 - Voltage Monitor Interrupt Type Select"]
    #[inline(always)]
    #[must_use]
    pub fn irqsel(&mut self) -> IRQSEL_W<LVDCR1_SPEC, 2> {
        IRQSEL_W::new(self)
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
#[doc = "Voltage Monitor %s Circuit Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvdcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvdcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVDCR1_SPEC;
impl crate::RegisterSpec for LVDCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvdcr1::R`](R) reader structure"]
impl crate::Readable for LVDCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lvdcr1::W`](W) writer structure"]
impl crate::Writable for LVDCR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVD%sCR1 to value 0x01"]
impl crate::Resettable for LVDCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
