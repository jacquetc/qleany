#pragma once

// example of using a model
// #include "my_list_model.h"
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

  private:
    Ui::MainWindow *ui;
    // example of using a model
    // FrontEnds::Presenter::MyListModel *m_myModel;
};