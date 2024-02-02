#pragma once

#include "interactor_export.h"
#include <QObject>
#include <qleany/contracts/repository/interface_repository_provider.h>

namespace Simple::Interactor
{

class SIMPLEEXAMPLE_INTERACTOR_EXPORT InteractorRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit InteractorRegistration(QObject *parent,
                                    Qleany::Contracts::Repository::InterfaceRepositoryProvider *repositoryProvider);
    ~InteractorRegistration();

  private:
};

} // namespace Simple::Interactor