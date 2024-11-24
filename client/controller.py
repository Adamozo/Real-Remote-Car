from pyPS4Controller.controller import Controller
from config import config


class MyController(Controller):

    def __init__(self, **kwargs):
        super().__init__(**kwargs)

    def on_x_press(self):
        pass

    def on_x_release(self):
        pass

    def on_circle_press(self):
        pass

    def on_circle_release(self):
        pass

    def on_triangle_press(self):
        pass

    def on_triangle_release(self):
        pass

    def on_square_press(self):
        pass

    def on_square_release(self):
        pass

    def on_L1_press(self):
        print("L1 press")

    def on_L1_release(self):
        print("L1 relese")

    def on_L2_press(self, value):
        print("L2 press: ", value)

    def on_L2_release(self):
        print("L2 relese")

    def on_R1_press(self):
        print("R1 press")

    def on_R1_release(self):
        print("R1 relese")

    def on_R2_press(self, value):
        print("R2 press: ", value)

    def on_R2_release(self):
        print("R2 relese")

    def on_up_arrow_press(self):
        pass

    def on_up_arrow_release(self):
        pass

    def on_down_arrow_press(self):
        pass

    def on_down_arrow_release(self):
        pass

    def on_left_arrow_press(self):
        pass

    def on_left_arrow_release(self):
        pass

    def on_right_arrow_press(self):
        pass

    def on_right_arrow_release(self):
        pass

    def on_options_press(self):
        pass

    def on_options_release(self):
        pass

    def on_share_press(self):
        pass

    def on_share_release(self):
        pass

    def on_L3_press(self):
        pass

    def on_L3_release(self):
        pass

    def on_L3_left(self, value):
        pass

    def on_R3_x_at_rest(self):
        pass

    def on_R3_y_at_rest(self):
        pass

    def on_R3_up(self, value):
        pass

    def on_R3_right(self, value):
        pass

    def on_R3_release(self):
        pass

    def on_playstation_button_press(self):
        pass

    def on_playstation_button_release(self):
        pass

    def on_trackpad_press(self):
        pass

    def on_trackpad_release(self):
        pass

    def on_joy_left_x(self, value):
        pass

    def on_joy_left_y(self, value):
        pass

    def on_joy_right_x(self, value):
        pass

    def on_joy_right_y(self, value):
        pass

    def on_L3_up(self, value):
        pass

    def on_L3_down(self, value):
        pass


controller = MyController(interface=config.CONTROLLER_INTERFACE,
                          connecting_using_ds4drv=False)
controller.listen(timeout=60)
