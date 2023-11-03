#pragma once

#include "car_list_model.h"
#include "passenger_list_model_from_car_passengers.h"
#include <QCoroTask>
#include <QMainWindow>

namespace Ui
{
class MainWindow;
}

class MainWindow : public QMainWindow
{
    Q_OBJECT

  public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow();
    QCoro::Task<> init();

  private:
    Ui::MainWindow *ui;
    Simple::Presenter::PassengerListModelFromCarPassengers *m_passengerModelFromCarPassengers;
    Simple::Presenter::CarListModel *m_carModel;
};
