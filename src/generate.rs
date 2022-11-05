/**
 * 性別。
 *
 * 國民身分證統一編號：男性為**1**；女性為**2**。
 * 新式外來人口統一證號：男性為**8**；女性為**9**。
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Sex {
    /**
     * 男性。
     */
    Male,
    /**
     * 女性。
     */
    Female,
}
