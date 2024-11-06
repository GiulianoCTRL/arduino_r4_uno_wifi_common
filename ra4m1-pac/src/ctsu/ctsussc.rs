#[doc = "Register `CTSUSSC` reader"]
pub type R = crate::R<CTSUSSC_SPEC>;
#[doc = "Register `CTSUSSC` writer"]
pub type W = crate::W<CTSUSSC_SPEC>;
#[doc = "Field `CTSUSSDIV` reader - CTSU Spectrum Diffusion Frequency Division Setting"]
pub type CTSUSSDIV_R = crate::FieldReader<CTSUSSDIV_A>;
#[doc = "CTSU Spectrum Diffusion Frequency Division Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUSSDIV_A {
    #[doc = "0: 4.00 &lt;= fb"]
    _0000 = 0,
    #[doc = "1: 2.00 &lt;= fb &lt; 4.00"]
    _0001 = 1,
    #[doc = "2: 1.33 &lt;= fb &lt; 2.00"]
    _0010 = 2,
    #[doc = "3: 1.00 &lt;= fb &lt; 1.33"]
    _0011 = 3,
    #[doc = "4: 0.80 &lt;= fb &lt; 1.00"]
    _0100 = 4,
    #[doc = "5: 0.67 &lt;= fb &lt; 0.80"]
    _0101 = 5,
    #[doc = "6: 0.57 &lt;= fb &lt; 0.67"]
    _0110 = 6,
    #[doc = "7: 0.50 &lt;= fb &lt; 0.57"]
    _0111 = 7,
    #[doc = "8: 0.44 &lt;= fb &lt; 0.50"]
    _1000 = 8,
    #[doc = "9: 0.40 &lt;= fb &lt; 0.44"]
    _1001 = 9,
    #[doc = "10: 0.36 &lt;= fb &lt; 0.40"]
    _1010 = 10,
    #[doc = "11: 0.33 &lt;= fb &lt; 0.36"]
    _1011 = 11,
    #[doc = "12: 0.31 &lt;= fb &lt; 0.33"]
    _1100 = 12,
    #[doc = "13: 0.29 &lt;= fb &lt; 0.31"]
    _1101 = 13,
    #[doc = "14: 0.27 &lt;= fb &lt; 0.29"]
    _1110 = 14,
    #[doc = "15: fb &lt; 0.27"]
    _1111 = 15,
}
impl From<CTSUSSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUSSDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUSSDIV_A {
    type Ux = u8;
}
impl CTSUSSDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSUSSDIV_A {
        match self.bits {
            0 => CTSUSSDIV_A::_0000,
            1 => CTSUSSDIV_A::_0001,
            2 => CTSUSSDIV_A::_0010,
            3 => CTSUSSDIV_A::_0011,
            4 => CTSUSSDIV_A::_0100,
            5 => CTSUSSDIV_A::_0101,
            6 => CTSUSSDIV_A::_0110,
            7 => CTSUSSDIV_A::_0111,
            8 => CTSUSSDIV_A::_1000,
            9 => CTSUSSDIV_A::_1001,
            10 => CTSUSSDIV_A::_1010,
            11 => CTSUSSDIV_A::_1011,
            12 => CTSUSSDIV_A::_1100,
            13 => CTSUSSDIV_A::_1101,
            14 => CTSUSSDIV_A::_1110,
            15 => CTSUSSDIV_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "4.00 &lt;= fb"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CTSUSSDIV_A::_0000
    }
    #[doc = "2.00 &lt;= fb &lt; 4.00"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CTSUSSDIV_A::_0001
    }
    #[doc = "1.33 &lt;= fb &lt; 2.00"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CTSUSSDIV_A::_0010
    }
    #[doc = "1.00 &lt;= fb &lt; 1.33"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == CTSUSSDIV_A::_0011
    }
    #[doc = "0.80 &lt;= fb &lt; 1.00"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CTSUSSDIV_A::_0100
    }
    #[doc = "0.67 &lt;= fb &lt; 0.80"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == CTSUSSDIV_A::_0101
    }
    #[doc = "0.57 &lt;= fb &lt; 0.67"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == CTSUSSDIV_A::_0110
    }
    #[doc = "0.50 &lt;= fb &lt; 0.57"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == CTSUSSDIV_A::_0111
    }
    #[doc = "0.44 &lt;= fb &lt; 0.50"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CTSUSSDIV_A::_1000
    }
    #[doc = "0.40 &lt;= fb &lt; 0.44"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == CTSUSSDIV_A::_1001
    }
    #[doc = "0.36 &lt;= fb &lt; 0.40"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == CTSUSSDIV_A::_1010
    }
    #[doc = "0.33 &lt;= fb &lt; 0.36"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == CTSUSSDIV_A::_1011
    }
    #[doc = "0.31 &lt;= fb &lt; 0.33"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == CTSUSSDIV_A::_1100
    }
    #[doc = "0.29 &lt;= fb &lt; 0.31"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == CTSUSSDIV_A::_1101
    }
    #[doc = "0.27 &lt;= fb &lt; 0.29"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == CTSUSSDIV_A::_1110
    }
    #[doc = "fb &lt; 0.27"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == CTSUSSDIV_A::_1111
    }
}
#[doc = "Field `CTSUSSDIV` writer - CTSU Spectrum Diffusion Frequency Division Setting"]
pub type CTSUSSDIV_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O, CTSUSSDIV_A>;
impl<'a, REG, const O: u8> CTSUSSDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4.00 &lt;= fb"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0000)
    }
    #[doc = "2.00 &lt;= fb &lt; 4.00"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0001)
    }
    #[doc = "1.33 &lt;= fb &lt; 2.00"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0010)
    }
    #[doc = "1.00 &lt;= fb &lt; 1.33"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0011)
    }
    #[doc = "0.80 &lt;= fb &lt; 1.00"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0100)
    }
    #[doc = "0.67 &lt;= fb &lt; 0.80"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0101)
    }
    #[doc = "0.57 &lt;= fb &lt; 0.67"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0110)
    }
    #[doc = "0.50 &lt;= fb &lt; 0.57"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0111)
    }
    #[doc = "0.44 &lt;= fb &lt; 0.50"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1000)
    }
    #[doc = "0.40 &lt;= fb &lt; 0.44"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1001)
    }
    #[doc = "0.36 &lt;= fb &lt; 0.40"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1010)
    }
    #[doc = "0.33 &lt;= fb &lt; 0.36"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1011)
    }
    #[doc = "0.31 &lt;= fb &lt; 0.33"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1100)
    }
    #[doc = "0.29 &lt;= fb &lt; 0.31"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1101)
    }
    #[doc = "0.27 &lt;= fb &lt; 0.29"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1110)
    }
    #[doc = "fb &lt; 0.27"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1111)
    }
}
impl R {
    #[doc = "Bits 8:11 - CTSU Spectrum Diffusion Frequency Division Setting"]
    #[inline(always)]
    pub fn ctsussdiv(&self) -> CTSUSSDIV_R {
        CTSUSSDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - CTSU Spectrum Diffusion Frequency Division Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ctsussdiv(&mut self) -> CTSUSSDIV_W<CTSUSSC_SPEC, 8> {
        CTSUSSDIV_W::new(self)
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
#[doc = "CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsussc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsussc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUSSC_SPEC;
impl crate::RegisterSpec for CTSUSSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctsussc::R`](R) reader structure"]
impl crate::Readable for CTSUSSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsussc::W`](W) writer structure"]
impl crate::Writable for CTSUSSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUSSC to value 0"]
impl crate::Resettable for CTSUSSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
