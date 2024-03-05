#include "mainwindow.h"
#include "ui_mainwindow.h"

// example of using an interactor
// #include "my_feature/my_feature_interactor.h"
// example of using a 'single', which represents one entity
// #include "single_my_entity.h"

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent), ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    
    // auto *myfeatureInteractor = FrontEnds::Interactor::MyFeature::MyFeatureInteractor::instance();
    // auto createMyFeatureDTO = myfeatureInteractor->getCreateDTO();
    // createMyFeatureDTO.setContent("Example myfeature 1");
    // const auto &myfeatureDto = co_await myfeatureInteractor->create(createMyFeatureDTO);


    // auto *singleMyEntity = new FrontEnds::Presenter::SingleMyEntity(this);
    // singleMyEntity->setId(0);

}

MainWindow::~MainWindow()
{
    delete ui;
}