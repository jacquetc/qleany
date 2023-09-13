#pragma once

#include "passenger_list_model.h"
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
    PassengerListModel *m_passengerModel;
};
