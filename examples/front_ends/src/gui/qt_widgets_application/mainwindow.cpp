// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#include "mainwindow.h"
#include "ui_mainwindow.h"

// example of using an controller
// #include "my_feature/my_feature_controller.h"
// example of using a 'single', which represents one entity
// #include "single_my_entity.h"

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent), ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    
    // auto *myfeatureController = FrontEnds::Controller::MyFeature::MyFeatureController::instance();
    // auto createMyFeatureDTO = myfeatureController->getCreateDTO();
    // createMyFeatureDTO.setContent("Example myfeature 1");
    // const auto &myfeatureDto = co_await myfeatureController->create(createMyFeatureDTO);


    // auto *singleMyEntity = new FrontEnds::Presenter::SingleMyEntity(this);
    // singleMyEntity->setId(0);

}

MainWindow::~MainWindow()
{
    delete ui;
}