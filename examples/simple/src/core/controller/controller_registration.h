#pragma once

#include "controller_export.h"
#include <QObject>
#include <qleany/contracts/repository/interface_repository_provider.h>

namespace Simple::Controller
{

class SIMPLEEXAMPLE_CONTROLLER_EXPORT ControllerRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit ControllerRegistration(QObject *parent,
                                    Qleany::Contracts::Repository::InterfaceRepositoryProvider *repositoryProvider);
    ~ControllerRegistration();

  private:
};

} // namespace Simple::Controller