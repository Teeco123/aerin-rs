use aerin_rs::math::vec2::Vec2;
use glam::Vec2 as GVec2;

#[test]
fn vec2_addition() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(4.0, 5.0);

    let g_v1 = GVec2::new(1.0, 2.0);
    let g_v2 = GVec2::new(4.0, 5.0);

    let r = v1 + v2;
    let g_r = g_v1 + g_v2;

    assert_eq!(r.x, g_r.x);
    assert_eq!(r.y, g_r.y);
}
