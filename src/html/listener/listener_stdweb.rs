// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
impl_action! {
    onclick(event: ClickEvent) -> ClickEvent => |_, event| { event }
    ondoubleclick(event: DoubleClickEvent) -> DoubleClickEvent => |_, event| { event }
    onkeypress(event: KeyPressEvent) -> KeyPressEvent => |_, event| { event }
    onkeydown(event: KeyDownEvent) -> KeyDownEvent => |_, event| { event }
    onkeyup(event: KeyUpEvent) -> KeyUpEvent => |_, event| { event }
    onmousemove(event: MouseMoveEvent) -> MouseMoveEvent => |_, event| { event }
    onmousedown(event: MouseDownEvent) -> MouseDownEvent => |_, event| { event }
    onmouseup(event: MouseUpEvent) -> MouseUpEvent => |_, event| { event }
    onmouseover(event: MouseOverEvent) -> MouseOverEvent => |_, event| { event }
    onmouseout(event: MouseOutEvent) -> MouseOutEvent => |_, event| { event }
    onmouseenter(event: MouseEnterEvent) -> MouseEnterEvent => |_, event| { event }
    onmouseleave(event: MouseLeaveEvent) -> MouseLeaveEvent => |_, event| { event }
    onmousewheel(event: MouseWheelEvent) -> MouseWheelEvent => |_, event| { event }
    ongotpointercapture(event: GotPointerCaptureEvent) -> GotPointerCaptureEvent => |_, event| { event }
    onlostpointercapture(event: LostPointerCaptureEvent) -> LostPointerCaptureEvent => |_, event| { event }
    onpointercancel(event: PointerCancelEvent) -> PointerCancelEvent => |_, event| { event }
    onpointerdown(event: PointerDownEvent) -> PointerDownEvent => |_, event| { event }
    onpointerenter(event: PointerEnterEvent) -> PointerEnterEvent => |_, event| { event }
    onpointerleave(event: PointerLeaveEvent) -> PointerLeaveEvent => |_, event| { event }
    onpointermove(event: PointerMoveEvent) -> PointerMoveEvent => |_, event| { event }
    onpointerout(event: PointerOutEvent) -> PointerOutEvent => |_, event| { event }
    onpointerover(event: PointerOverEvent) -> PointerOverEvent => |_, event| { event }
    onpointerup(event: PointerUpEvent) -> PointerUpEvent => |_, event| { event }
    onscroll(event: ScrollEvent) -> ScrollEvent => |_, event| { event }
    onblur(event: BlurEvent) -> BlurEvent => |_, event| { event }
    onfocus(event: FocusEvent) -> FocusEvent => |_, event| { event }
    onsubmit(event: SubmitEvent) -> SubmitEvent => |_, event| { event }
    ondragstart(event: DragStartEvent) -> DragStartEvent => |_, event| { event }
    ondrag(event: DragEvent) -> DragEvent => |_, event| { event }
    ondragend(event: DragEndEvent) -> DragEndEvent => |_, event| { event }
    ondragenter(event: DragEnterEvent) -> DragEnterEvent => |_, event| { event }
    ondragleave(event: DragLeaveEvent) -> DragLeaveEvent => |_, event| { event }
    ondragover(event: DragOverEvent) -> DragOverEvent => |_, event| { event }
    ondragexit(event: DragExitEvent) -> DragExitEvent => |_, event| { event }
    ondrop(event: DragDropEvent) -> DragDropEvent => |_, event| { event }
    oncontextmenu(event: ContextMenuEvent) -> ContextMenuEvent => |_, event| { event }
    oninput(event: InputEvent) -> InputData => |this: &Element, _| { oninput_handler(this) }
    onchange(event: ChangeEvent) -> ChangeData => |this: &Element, _| { onchange_handler(this) }
    ontouchcancel(event: TouchCancel) -> TouchCancel => |_, event| { event }
    ontouchend(event: TouchEnd) -> TouchEnd => |_, event| { event }
    ontouchenter(event: TouchEnter) -> TouchEnter => |_, event| { event }
    ontouchmove(event: TouchMove) -> TouchMove => |_, event| { event }
    ontouchstart(event: TouchStart) -> TouchStart => |_, event| { event }
}
